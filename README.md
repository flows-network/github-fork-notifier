# <p align="center">GitHub Fork Notifier</p>

<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/new">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>


[Deploy this function on flows.network](#deploy-the-github-fork-notifier), and you will get an automated workflow: When someone forks your GitHub repo, you will get a Slack message, meawhile this fork record is saved to a form on Airtable. It helps DevRels and Community Managers to stay on top of the growth of their GitHub communities. 

![Github fork to slack](https://user-images.githubusercontent.com/45785633/227115653-935c616b-2881-4832-a53c-850f63a0f322.png)

![Save GitHub fork to Airtable ](https://user-images.githubusercontent.com/45785633/227122019-258041fc-ce2c-4819-9648-8fc51fee5782.png)


## Prerequisite 
* A Slack Account
* An Airtable account, an [Airtable API Key](https://airtable.com/account) and also a base to record the new forks. For the form, you can simply [copy the template base](https://airtable.com/shrwhFUgnz97Uf4nr). 

## Deploy the GitHub fork notifier

To create this fork notifier, we will use [flows.network](https://flows.network/), a serverless platform that makes deploying your own app quick and easy in just 3 steps.

### Fork this repo and cutomize the code

Fork [this repo](https://github.com/flows-network/chatgpt-github-app/) and customize the code based on your needs. Since this function involes three SaaS integrations, so we need to change the code as the following.

1. GitHub: 
* Replace `alabulei1` with your own personal id；
* Replace `WasmEdge` and `WasmEdge` with your own GitHub repo you want to monitor for the fork data；

```
pub async fn run() {
    let login = "alabulei1";
    let owner = "WasmEdge";
    let repo = "WasmEdge";

    listen_to_event(owner, repo, vec!["fork"], handler).await;
}
```
2. Airtable: Change the parameters for Airtable based on the code comment. This is where you save the new fork.
```
async fn handler(payload: EventPayload) {
    let account: &str = "github"; // The name that you will name your Airtable API key in the SaaS configuration step. You can get this from Airtable.
    let base_id: &str = "appNEswczILgUsxML"; // This is the base where you want to record new forks. You can get this from Airtable easily. Please refer to https://support.airtable.com/docs/finding-airtable-ids#finding-ids-in-airtable-api
    let table_name: &str = "fork"; // This the table name in the above base. If you're using the template that we provide, then don't need to change this.
````

3. Slack: Replace `secondstate` and `github-status` with your own Slack workaspace and channel. This is where you get the new fork message.

```
let text = format!("{} forked your {}\n{}", name, html_url, time);
send_message_to_channel("secondstate", "github-status", text);
```

### Deploy the code on flow.network

Next, let us deploy this repo on flows.network.

1. Sign up for an accoun on [flows.network](https://flows.network/). It's a free platform for deploying automation workflows.
2. Click on the "Create a Flow" button to start deploying this function
3. Authenticate the [flows.network](https://flows.network/) to access the `github-fork-notifier` repo you just forked. 

<img width="886" alt="image" src="https://user-images.githubusercontent.com/45785633/227131173-26a1da68-74d0-479e-88d3-1184f6db0755.png">

4. Click the Deploy button to deploy your function.

### Configure SaaS integrations

After that, the flows.network will direct you to configure the SaaS integrations required by your flow.

<img width="1452" alt="image" src="https://user-images.githubusercontent.com/45785633/227131712-f6356a60-830d-4980-b563-458da0816333.png">

Here we can see, we need to configue three SaaS integrations.

1. Click the "Connect/+ Add new authentication" button to authenticate your Slack account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on your workspace. This workspace is the one you changed in the code above.

2. Click the "Connect/+ Add new authentication" button to authenticate your Airtable account. You'll be redirected to a new page where you can copy and paste your Airtable API key and then name the key. **Note that the name you enter here should be the same as the name you changed in the code above.**

<img width="741" alt="image" src="https://user-images.githubusercontent.com/45785633/227132305-b093dded-6569-4c29-8026-55a3ec9bc62b.png">

3. Click the "Connect/+ Add new authentication" button to authenticate your GitHub account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on the repo that you changed in the code above.

After that, click the Check button to see your flow details. As soon as the flow function's status becomes `ready` and the flow's status becomes `running`, the flow goes live. Whenever a GitHub user forks your repo, you will get a Slack notifitaion and a new record in your Airtable.
<img width="1183" alt="image" src="https://user-images.githubusercontent.com/45785633/227132839-d4008845-4a67-46dc-af14-90e06ad96e12.png">

> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!

