# Decentralized Freelance Platform

A smart contract-based freelancing system on the Internet Computer.

## About

This project implements a basic freelance platform using the Internet Computer Protocol. It allows users to register, post jobs, and view job listings.

## Setup

### Prerequisites

1. **Install DFX**: Ensure you have the DFX SDK installed. You can follow the installation instructions from the [DFINITY documentation](https://sdk.dfinity.org/docs/quickstart/quickstart.html).

2. **Clone the Repository**: Clone the project repository to your local machine.

   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```

### Starting the Local Internet Computer

1. **Start DFX**: Run the following command to start the local Internet Computer environment in the background.

   ```bash
   dfx start --background
   ```

### Deploying the Canister

1. **Deploy the Canister**: Use the following command to deploy your canister.

   ```bash
   dfx deploy
   ```

   This command will compile the Rust code, create the necessary canisters, and deploy them to the local network.

## Usage

### Interacting with the Canister

You can interact with the canister using the Candid UI or the command line interface (CLI).

### Using the Candid UI

1. **Access the Candid UI**: After deploying the canister, you can access the Candid UI by going to the Backend canister via Candid interface link provided in the terminal or navigating to the following URL in your web browser:

   ```
   http://localhost:8000/?canisterId=<your_canister_id>
   ```

   Replace `<your_canister_id>` with the actual canister ID that was output during the deployment process.

2. **Interact with the Methods**: In the Candid UI, you will see a list of available methods. You can test the methods by providing the required parameters and clicking the "Call" button. Here are some examples:

- **Post a Job**:
     - Method: `post_job`
     - Parameters: `("client_id", "Job Title", "Description", 1000)`
     - Click "Call" to post the job.

   - **Get Job Details**:
     - Method: `get_job`
     - Parameters: `("job_id")`
     - Click "Call" to retrieve job details.

   - **List Open Jobs**:
     - Method: `list_open_jobs`
     - Click "Call" to see all open jobs.

   - **Get All Jobs**:
     - Method: `get_all_jobs`
     - Click "Call" to retrieve all jobs.

   - **Update Job Status**:
     - Method: `update_job_status`
     - Parameters: `("job_id", JobStatus)`
     - Click "Call" to update the status of a job.

   - **Complete a Job**:
     - Method: `complete_job`
     - Parameters: `("job_id")`
     - Click "Call" to mark a job as completed.

   - **Resolve a Dispute**:
     - Method: `resolve_dispute`
     - Parameters: `("job_id", "user_id", bool)`
     - Click "Call" to resolve a dispute for a job.

   - **Get User Reputation**:
     - Method: `get_user_reputation`
     - Parameters: `("user_id")`
     - Click "Call" to retrieve the reputation of a user.

   - **Calculate User Reputation**:
     - Method: `calculate_reputation`
     - Parameters: `("user_id")`
     - Click "Call" to calculate the reputation based on assigned and completed jobs.


### Using CLI Sample Tests

You can also interact with the canister using the command line interface (CLI). Here are some sample commands to test the methods:

1. **Register a User**:

   ```bash
   dfx canister call decentralized_freelance_backend register_user '("user_id")'
   ```

2. **Post a Job**:

   ```bash
   dfx canister call decentralized_freelance_backend post_job '("client_id", "Job Title", "Description", 1000)'
   ```

3. **Get Job Details**:

   ```bash
   dfx canister call decentralized_freelance_backend get_job '("job_id")'
   ```

4. **List Open Jobs**:

   ```bash
   dfx canister call decentralized_freelance_backend list_open_jobs
   ```

5. **Complete a Job**:

   ```bash
   dfx canister call decentralized_freelance_backend complete_job '("job_id")'
   ```

6. **Resolve a Dispute**:

   ```bash
   dfx canister call decentralized_freelance_backend resolve_dispute '("job_id", "user_id", true)'
   ```

7. **Calculate User Reputation**:

   ```bash
   dfx canister call decentralized_freelance_backend calculate_reputation '("user_id")'
   ```

## Project Structure

- `src/decentralized_freelance_backend/`: Canister code
- `src/decentralized_freelance_backend/decentralized_freelance_backend.did`: Candid interface
- `dfx.json`: Project configuration

## Development

To make changes:

1. Edit the code in `src/decentralized_freelance_backend/src/lib.rs`.
2. Rebuild and redeploy:

   ```bash
   dfx build
   dfx canister install decentralized_freelance_backend --mode upgrade
   ```
   
## License

This project is under the MIT License.