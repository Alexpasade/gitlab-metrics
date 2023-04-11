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

1. source_branch: The name of the source branch you want to analyze (e.g., staging).
1. target_branch: The name of the target branch you want to analyze (e.g., production).
1. gitlab_token: Your GitLab personal access token. You can create a new token if you don't have one.
1. project_id: The GitLab project ID you want to analyze. You can find this on your project's GitLab page.

The application will process GitLab merge requests for the specified project and calculate the time difference between merging the source branch into the target branch.

Enjoy using the CRONOS CLI app!


### License

MIT License

Copyright (c) 2023 Alejandro Gómez Pasadé

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
