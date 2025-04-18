use reqwest::header::HeaderMap;
use serde::Deserialize;
mod task;
use task::Task;
mod project;
use project::Project;

use crate::filter;
use crate::project::Project as ProjectTrait;
use crate::task::Provider;
use crate::task::Task as TaskTrait;

use async_trait::async_trait;

const BASE_URL: &str = "https://todoist.com/api/v1";
const PROVIDER_NAME: &str = "Todoist";

pub struct Todoist {
    default_header: HeaderMap,
    client: reqwest::Client,
}

impl Todoist {
    pub fn new(api_key: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", api_key).parse().unwrap(),
        );
        Self {
            default_header: headers,
            client: reqwest::Client::new(),
        }
    }

    async fn uncompleted_tasks(
        &self,
        project: &Option<String>,
    ) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let mut result: Vec<Task> = Vec::new();

        let mut cursor = None;

        let query = {
            let mut v = vec![String::from("limit=200")];

            if let Some(p) = project {
                v.push(format!("project_id={p}"));
            }

            v
        };

        loop {
            let mut q = query.clone();
            if let Some(c) = cursor {
                q.push(format!("cursor={c}"));
            }

            let mut resp = self
                .client
                .get(format!("{BASE_URL}/tasks?{}", q.join("&")))
                .headers(self.default_header.clone())
                .send()
                .await?
                .json::<TaskResponse>()
                .await?;

            result.append(&mut resp.results);

            if resp.next_cursor.is_none() {
                break;
            }

            cursor = resp.next_cursor;
        }

        Ok(result)
    }

    async fn completed_tasks(
        &self,
        project: &Option<String>,
    ) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let mut result: Vec<Task> = Vec::new();

        let mut cursor = None;

        let query = {
            let mut v = vec![
                String::from("limit=200"),
                format!(
                    "since={}",
                    chrono::Utc::now()
                        .checked_sub_days(chrono::Days::new(7))
                        .unwrap()
                        .format("%Y-%m-%dT%H:%M:%SZ")
                ),
                format!("until={}", chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ")),
            ];

            if let Some(p) = project {
                v.push(format!("project_id={p}"));
            }

            v
        };

        #[allow(dead_code)]
        #[derive(Deserialize, Debug)]
        struct Response {
            pub items: Vec<Task>,
            pub next_cursor: Option<String>,
        }

        loop {
            let mut q = query.clone();
            if let Some(c) = cursor {
                q.push(format!("cursor={c}"));
            }
            println!(
                "{BASE_URL}/tasks/completed/by_completion_date?{}",
                &q.join("&")
            );
            let mut resp = self
                .client
                .get(format!(
                    "{BASE_URL}/tasks/completed/by_due_date?{}",
                    &q.join("&")
                ))
                .headers(self.default_header.clone())
                .send()
                .await?
                .json::<Response>()
                .await?;

            result.append(&mut resp.items);

            if resp.next_cursor.is_none() {
                break;
            }

            cursor = resp.next_cursor;
        }

        Ok(result)
    }

    async fn tasks_by_filter(
        &self,
        project: &Option<String>,
        f: &filter::Filter,
    ) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let mut result: Vec<Task> = Vec::new();

        let mut cursor = None;

        let mut todoist_query = String::new();
        if f.today {
            todoist_query.push_str("today")
        }

        if todoist_query.is_empty() {
            todoist_query = "all".to_string();
        }

        let query = {
            let mut v = vec![String::from("limit=200"), format!("query={todoist_query}")];

            if let Some(p) = project {
                v.push(format!("project_id={p}"));
            }

            v
        };

        #[allow(dead_code)]
        #[derive(Deserialize, Debug)]
        struct Response {
            pub results: Vec<Task>,
            pub next_cursor: Option<String>,
        }

        loop {
            let mut q = query.clone();
            if let Some(c) = cursor {
                q.push(format!("cursor={c}"));
            }
            let mut resp = self
                .client
                .get(format!("{BASE_URL}/tasks/filter?{}", &q.join("&")))
                .headers(self.default_header.clone())
                .send()
                .await?
                .json::<Response>()
                .await?;

            result.append(&mut resp.results);

            if resp.next_cursor.is_none() {
                break;
            }

            cursor = resp.next_cursor;
        }

        Ok(result)
    }
    pub async fn tasks(
        &self,
        project: &Option<String>,
        f: &filter::Filter,
    ) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let mut result: Vec<Task> = Vec::new();

        if f.states.contains(&filter::FilterState::Uncompleted) {
            result.append(&mut self.uncompleted_tasks(project).await?);
        }

        if f.states.contains(&filter::FilterState::Completed) {
            result.append(&mut self.completed_tasks(project).await?);
        }

        Ok(result)
    }

    pub async fn projects(&self) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        let mut result: Vec<Project> = Vec::new();

        let mut cursor = None;

        loop {
            let mut query: String = String::from("?limit=200");
            if let Some(c) = cursor {
                query.push_str(format!("&cursor={c}").as_str());
            }

            let mut resp = self
                .client
                .get(format!("{BASE_URL}/projects{query}"))
                .headers(self.default_header.clone())
                .send()
                .await?
                .json::<ProjectResponse>()
                .await?;

            result.append(&mut resp.results);

            if resp.next_cursor.is_none() {
                break;
            }

            cursor = resp.next_cursor;
        }

        Ok(result)
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct TaskResponse {
    pub results: Vec<Task>,
    pub next_cursor: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct ProjectResponse {
    pub results: Vec<Project>,
    pub next_cursor: Option<String>,
}

pub struct TodoistProvider {
    t: Todoist,
}

impl TodoistProvider {
    pub fn new(t: Todoist) -> Self {
        Self { t }
    }
}

#[async_trait]
impl Provider for TodoistProvider {
    fn name(&self) -> String {
        PROVIDER_NAME.to_string()
    }

    async fn tasks(
        &self,
        f: &filter::Filter,
    ) -> Result<Vec<Box<dyn TaskTrait>>, Box<dyn std::error::Error>> {
        let tasks = self.t.tasks_by_filter(&None, f).await?;
        let mut result: Vec<Box<dyn TaskTrait>> = Vec::new();
        for t in tasks {
            result.push(Box::new(t));
        }
        Ok(result)
    }

    async fn projects(&self) -> Result<Vec<Box<dyn ProjectTrait>>, Box<dyn std::error::Error>> {
        let projects = self.t.projects().await?;
        let mut result: Vec<Box<dyn ProjectTrait>> = Vec::new();
        for p in projects {
            result.push(Box::new(p));
        }

        Ok(result)
    }
}
