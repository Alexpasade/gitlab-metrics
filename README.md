# Gitlab Metric Deploys CLI Application
This is a simple command-line interface (CLI) application that helps you to calculate the time it takes to merge GitLab merge requests. It takes four input parameters: source_branch, target_branch, gitlab_token, and project_id.

## Prerequisites
Docker: You'll need Docker installed on your system

## Building and Running the Application

1. Build the Docker image
```bash
docker build -t metric_gitlab_deploys .
```
2. Execute application
```bash
docker run --rm -it metric_gitlab_deploys
```

## Usage

The application takes the following input parameters:

1. source_branch: The name of the source branch you want to analyze (e.g., main).
1. target_branch: The name of the target branch you want to analyze (e.g., production).
1. gitlab_token: Your GitLab personal access token. You can create a new token if you don't have one.
1. project_id: The GitLab project ID you want to analyze. You can find this on your project's GitLab page.

The application will process last 20 GitLab merge requests for the specified project and calculate the time difference between merging the source branch into the target branch for every single merge and then it will show you the average time of the merges.

Enjoy using the CRONOS CLI app!

