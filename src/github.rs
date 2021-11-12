use core::fmt;

use octocrab::{Octocrab,params,models};
use chrono::{DateTime, Duration, Utc};
use timeago;

use crate::config::GhConfig;

pub struct GhRunner {
    octocrab: Octocrab,
}

pub struct GhRepoId {
    user: String,
    repo: String,
}

pub struct GhPullRequest {
    pub user: String,
    pub url: String,
    pub created_or_updated: DateTime<Utc>,
}

impl fmt::Display for GhPullRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tf = timeago::Formatter::new();
        let duration = tf.convert((Utc::now() - self.created_or_updated).to_std().unwrap());
        write!(f, " - {} @{} {}", self.url, self.user, duration)
    }
}

impl GhRepoId {
    pub fn from_string(repo_string: &String) -> Self {
        let pair: Vec<&str> = repo_string.split("/").collect();
        if pair.len() != 2 {
            panic!("Invalid repo name {}", repo_string)
        }
        Self {
            user: pair.get(0).unwrap().to_string(),
            repo: pair.get(1).unwrap().to_string(),
        }
    }
}

impl GhRunner {
    pub fn new(client: Octocrab) -> Self {
        Self {
            octocrab: client
        }
    }

    async fn query_for_repo(&self, repo: &GhRepoId, users: &Vec<String>) -> Vec<GhPullRequest> {
        let mut page = self.octocrab.pulls(&repo.user, &repo.repo).list()
        .state(params::State::Open)
        .sort(params::pulls::Sort::Created)
        .direction(params::Direction::Descending)
        .per_page(255)
        .send().await.unwrap();
        let mut ret :Vec<GhPullRequest> = Vec::new();
        loop  {
            for pull in &page {
                if pull.draft {
                    continue
                }
                if users.contains(&pull.user.login) {
                    let dt = match pull.updated_at {
                        Some(date) => date,
                        None => pull.created_at,
                    };

                    if pull.created_at > Utc::now() - Duration::weeks(1) {
                        ret.push(GhPullRequest{
                            user: pull.user.login.clone(),
                            url: pull.html_url.to_string(),
                            created_or_updated: dt,
                        })
                    }
                }
            };
            page = match self.octocrab.get_page::<models::pulls::PullRequest>(&page.next).await.unwrap() {
                Some(next_page) => next_page,
                None => break,
            }
        }
        ret
    }
    pub async fn query(&self, config: &GhConfig) -> Vec<GhPullRequest> {
        let mut results:Vec<GhPullRequest> = Vec::new();
        for repo in &config.repos {
            println!("Querying {}", repo.repo);
            let gh_repo = GhRepoId::from_string(&repo.repo);
            let res =self.query_for_repo(&gh_repo, &repo.users).await;
            results.extend(res)
        }
        results.sort_by(|a, b| b.created_or_updated.cmp(&a.created_or_updated));
        results
    }
}