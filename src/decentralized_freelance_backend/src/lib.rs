use candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

#[derive(CandidType, Deserialize, Clone)]
struct User {
	id: String,
	reputation: u32,
}

#[derive(CandidType, Deserialize, Clone)]
struct Job {
	id: String,
	title: String,
	description: String,
	budget: u32,
	client: String,
	freelancer: Option<String>,
	status: JobStatus,
}

#[derive(CandidType, Deserialize, Clone)]
enum JobStatus {
	Open,
	Assigned,
	Completed,
	Disputed,
}

type UserStore = HashMap<String, User>;
type JobStore = HashMap<String, Job>;

thread_local! {
	static USERS: RwLock<UserStore> = RwLock::new(HashMap::new());
	static JOBS: RwLock<JobStore> = RwLock::new(HashMap::new());
}

#[derive(Debug)]
enum CanisterError {
	UserAlreadyExists,
	JobNotFound,
	Other(String),
}

impl std::fmt::Display for CanisterError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CanisterError::UserAlreadyExists => write!(f, "User already exists"),
			CanisterError::JobNotFound => write!(f, "Job not found"),
			CanisterError::Other(msg) => write!(f, "{}", msg),
		}
	}
}

impl std::error::Error for CanisterError {}

#[update]
fn register_user(id: String) -> Result<(), CanisterError> {
	USERS.with(|users| {
		let mut users = users.write().unwrap();
		if users.contains_key(&id) {
			Err(CanisterError::UserAlreadyExists)
		} else {
			users.insert(id.clone(), User { id, reputation: 0 });
			Ok(())
		}
	})
}

#[update]
fn post_job(client: String, title: String, description: String, budget: u32) -> Result<String, CanisterError> {
	let job_id = Uuid::new_v4().to_string();
	let job = Job {
		id: job_id.clone(),
		title,
		description,
		budget,
		client,
		freelancer: None,
		status: JobStatus::Open,
	};
	
	JOBS.with(|jobs| {
		jobs.write().unwrap().insert(job_id.clone(), job);
	});
	
	Ok(job_id)
}

#[query]
fn get_job(job_id: String) -> Result<Job, CanisterError> {
	JOBS.with(|jobs| {
		jobs.read().unwrap().get(&job_id).cloned().ok_or(CanisterError::JobNotFound)
	})
}

#[query]
fn list_open_jobs() -> Vec<Job> {
	JOBS.with(|jobs| {
		jobs.read().unwrap()
			.values()
			.filter(|job| matches!(job.status, JobStatus::Open))
			.cloned()
			.collect()
	})
}