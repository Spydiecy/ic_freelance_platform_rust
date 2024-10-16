use candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use std::collections::HashMap;
use std::fmt;

#[derive(CandidType, Deserialize, Clone)]
struct User {
    id: String,
    reputation: u32,
    assigned_jobs: u32, // New field to track assigned jobs
    completed_jobs: u32, // New field to track completed jobs
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
    static USERS: std::cell::RefCell<UserStore> = std::cell::RefCell::new(HashMap::new());
    static JOBS: std::cell::RefCell<JobStore> = std::cell::RefCell::new(HashMap::new());
}

#[derive(Debug, CandidType, Deserialize)]
pub enum JobError {
    UserAlreadyExists,
    JobNotFound,
    InvalidJobStatus,
    InsufficientJobs,
    DisputeResolutionFailed,
    Other(String), // For any other errors
}

impl fmt::Display for JobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobError::UserAlreadyExists => write!(f, "User already exists"),
            JobError::JobNotFound => write!(f, "Job not found"),
            JobError::InvalidJobStatus => write!(f, "Invalid job status"),
            JobError::InsufficientJobs => write!(f, "Insufficient jobs for reputation calculation"),
            JobError::DisputeResolutionFailed => write!(f, "Failed to resolve dispute"),
            JobError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

#[update]
fn register_user(id: String) -> Result<(), JobError> {
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        if users.contains_key(&id) {
            return Err(JobError::UserAlreadyExists);
        } else {
            users.insert(id.clone(), User { id, reputation: 0, assigned_jobs: 0, completed_jobs: 0 });
            Ok(())
        }
    })
}

#[update]
fn post_job(client: String, title: String, description: String, budget: u32) -> Result<String, String> {
    let job_id = ic_cdk::api::time().to_string(); // Using timestamp as a simple unique ID
    let job = Job {
        id: job_id.clone(),
        title,
        description,
        budget,
        client: client.clone(), // Clone the client string here
        freelancer: None,
        status: JobStatus::Open,
    };
    
    JOBS.with(|jobs| {
        jobs.borrow_mut().insert(job_id.clone(), job);
    });
    
    // Increment the assigned jobs count for the client
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        if let Some(user) = users.get_mut(&client) {
            user.assigned_jobs += 1; // Increment assigned jobs
        }
    });
    
    Ok(job_id)
}

#[query]
fn get_job(job_id: String) -> Option<Job> {
    JOBS.with(|jobs| jobs.borrow().get(&job_id).cloned())
}

#[query]
fn list_open_jobs() -> Vec<Job> {
    JOBS.with(|jobs| {
        jobs.borrow()
            .values()
            .filter(|job| matches!(job.status, JobStatus::Open))
            .cloned()
            .collect()
    })
}

#[query]
fn get_all_jobs() -> Vec<Job> {
    JOBS.with(|jobs| {
        jobs.borrow()
            .values()
            .cloned()
            .collect()
    })
}

#[update]
fn update_job_status(job_id: String, new_status: JobStatus) -> Result<(), String> {
    JOBS.with(|jobs| {
        let mut jobs = jobs.borrow_mut();
        match jobs.get_mut(&job_id) {
            Some(job) => {
                job.status = new_status;
                Ok(())
            },
            None => Err("Job not found".to_string()),
        }
    })
}

#[update]
fn complete_job(job_id: String) -> Result<(), JobError> {
    JOBS.with(|jobs| {
        let mut jobs = jobs.borrow_mut();
        match jobs.get_mut(&job_id) {
            Some(job) => {
                // Increase the client's reputation
                USERS.with(|users| {
                    let mut users = users.borrow_mut();
                    if let Some(user) = users.get_mut(&job.client) {
                        user.reputation += 10; // Increase reputation by 10 points
                        user.completed_jobs += 1; // Increment completed jobs
                    }
                });
                // Mark the job as completed
                job.status = JobStatus::Completed;
                Ok(())
            },
            None => Err(JobError::JobNotFound),
        }
    })
}

#[update]
fn resolve_dispute(job_id: String, user_id: String, outcome: bool) -> Result<(), String> {
    JOBS.with(|jobs| {
        let mut jobs = jobs.borrow_mut();
        match jobs.get_mut(&job_id) {
            Some(job) => {
                // Adjust reputation based on dispute outcome
                USERS.with(|users| {
                    let mut users = users.borrow_mut();
                    if let Some(user) = users.get_mut(&user_id) {
                        if outcome {
                            user.reputation += 5; // Increase reputation if the user wins the dispute
                        } else {
                            user.reputation -= 5; // Decrease reputation if the user loses the dispute
                        }
                    }
                });
                Ok(())
            },
            None => Err("Job not found".to_string()),
        }
    })
}

#[query]
fn get_user_reputation(user_id: String) -> Option<u32> {
    USERS.with(|users| {
        users.borrow().get(&user_id).map(|user| user.reputation)
    })
}

// Add a method to calculate reputation based on assigned and completed jobs
#[query]
fn calculate_reputation(user_id: String) -> Result<String, JobError> {
    USERS.with(|users| {
        if let Some(user) = users.borrow().get(&user_id) {
            // Calculate reputation as a ratio of completed to assigned jobs
            if user.assigned_jobs > 0 {
                let reputation_percentage = (user.completed_jobs * 100) / user.assigned_jobs; // Reputation as a percentage
                Ok(format!("{}%", reputation_percentage)) // Return as a string with a percentage sign
            } else {
                Ok("0%".to_string()) // No jobs assigned means no reputation
            }
        } else {
            Err(JobError::Other("User not found".to_string())) // User not found
        }
    })
}
