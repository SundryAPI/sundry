<div align="center">
   <picture>
     <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/f72c2509-5812-47bc-9325-c3283ede6ef0">
     <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/f72c2509-5812-47bc-9325-c3283ede6ef0">
     <img alt="Logo" src="" width="680">
   </picture>
</div>

<p align="center">
   <p align="center"><b>Sundry - A Context API for LLMs</b></p>
</p>

<p align="center">
| <a href="https://sundry.readme.io/reference/getting-started"><b>Documentation</b></a> | <a href="https://postgresml.org/blog"><b>Blog</b></a> | <a href="https://discord.gg/DmyJP3qJ7U"><b>Discord</b></a> |
</p>

---


Sundry is an **intelligent context provider API** designed specifically for Large Language Models (LLMs). It connects to user data sources—like GitHub, Jira, Slack, and Office 365—and returns only the exact results the LLM requests. Sundry gives LLMs reliable access to the precise context they need, when they need it, without guesswork or manual context uploads.

**📕 Table of Contents**
- [🤖 What Is Sundry?](#-what-is-sundry)
- [🏆 Why Sundry?](#-why-sundry)
- [⚡ Key Features](#-key-features)
- [🚀 Get Started](#-get-started)
- [📘 Documentation](#-documentation)
- [🌐 Community](#-community)
- [🤝 Contributing](#-contributing)

# 🤖 What Is Sundry?

Sundry is an **intelligent context provider** built specifically to be used as a tool for Large Language Models (LLMs). Rather than functioning as a layer of “fuzzy” semantic search or a RAG solution, Sundry focuses on delivering **exact** information from real user data sources—like GitHub, Jira, Slack, and Office 365—whenever the LLM requests it. 

This direct approach lets LLMs decide **when and how** to gather context, and ensures that any retrieved results are precise and relevant, without relying on guesswork or approximate matching. Sundry acts as a dedicated tool for LLMs to retrieve factual, actionable data—empowering AI-driven applications to offer experiences akin to working with a fully informed team member.

# 🏆 Why Sundry?

Sundry shifts the context burden **from the user to the AI** by giving LLMs on-demand access to a unified, indexed pool of user data across multiple services (e.g., GitHub, Jira, Slack). As a result, the AI can:
- **Retrieve context proactively**: Instead of waiting for users to provide extensive details, LLMs can fetch relevant data whenever needed.
- **Provide real, exact results**: Sundry avoids guesswork or approximate matching by returning precise context—not RAG over user data.
- **Work like a true team member**: With reliable data at its fingertips, an LLM can hold more natural, informed conversations and make decisions based on actual records, not assumptions.

# ⚡ Key Features

- **Natural Language Data Access**  
  LLMs query user data in plain language (e.g., “What was my last GitHub issue?”), and Sundry returns the _exact_ information requested. No complex API structures or guesswork.

- **Real Actionable Results**  
  Sundry is NOT RAG over user data. It provides LLMs with _precise_ context, ensuring answers are based on real, verifiable data.

- **Unified Data Aggregation**  
  Connect multiple sources (GitHub, Jira, Slack, Office 365, etc.) in one place. LLMs query a _single_ endpoint, freeing you from juggling numerous integrations.

- **LLM-Optimized Responses**  
  All query responses come structured for easy consumption by AI models, including both the requested data and metadata about how the query was interpreted.

- **Data Privacy & Control**  
  Keep full visibility and control over what each connected application can access. (A self-service dashboard with fine-grained permissions is in progress.)

# 🚀 Get Started

You can run Sundry on your own infrastructure! Refer to our [SELF_HOSTING.md](./SELF_HOSTING.md) for detailed instructions on setting up and managing a self-hosted instance.

We also host! Below are the essential steps to begin using Sundry in your LLM-driven application:

1. **Generate an API Key**
   Log in to your [Sundry](https://getsundry.app/dashboard) dashboard to create an API key. Keep this key secure—never share it publicly.

2. **Users Connect Their Data Sources**
   Your end users will link their own services (GitHub, Slack, Jira, etc.) to Sundry. Once connected, their data is automatically indexed and ready to query.

3. **Explore the API Reference**
   Check the [API Reference](https://sundry.readme.io/reference/getting-started) to understand request parameters, authentication, and response formats.

4. **Integrate Sundry with Your LLM**
   Head over to the [LLM Integration Guide](https://sundry.readme.io/docs/overview) for best practices on incorporating Sundry queries directly into your AI application.

# 📘 Documentation

For comprehensive documentation, including API references, tutorials, and best practices, visit our [official documentation](https://sundry.readme.io/docs/getting-started).

# 🌐 Community

Join our community to get help, share ideas, and contribute:

- [Discord](https://discord.gg/DmyJP3qJ7U)
- [Twitter](https://x.com/postgresml)

# 🤝 Contributing

We welcome contributions to Sundry! Please read our [Contribution Guidelines](CONTRIBUTING.md) before submitting pull requests.

---

Sundry is maintained by [PostgresML](https://postgresml.org). For enterprise support and consulting services, please [contact us](https://postgresml.org/contact).
