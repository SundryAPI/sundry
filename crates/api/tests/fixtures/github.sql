--
-- PostgreSQL database dump
--

-- Dumped from database version 15.8 (Homebrew)
-- Dumped by pg_dump version 15.10 (Homebrew)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Data for Name: users; Type: TABLE DATA; Schema: github; Owner: silas
--

INSERT INTO github.users VALUES (19626586, 'dev1');
INSERT INTO github.users VALUES (1037150, 'sebzur');
INSERT INTO github.users VALUES (44410570, 'mattephi');
INSERT INTO github.users VALUES (48477665, 'koritsky');
INSERT INTO github.users VALUES (11238160, 'ElYaiko');
INSERT INTO github.users VALUES (13739892, 'pidgeon777');
INSERT INTO github.users VALUES (34910769, 'xinye0123');
INSERT INTO github.users VALUES (1893361, 'krukas');
INSERT INTO github.users VALUES (145156502, 'a-rbts');
INSERT INTO github.users VALUES (180627245, 'mergemoveagree');
INSERT INTO github.users VALUES (43817946, 'harisf3');
INSERT INTO github.users VALUES (40359643, 'PorcoRosso85');
INSERT INTO github.users VALUES (6314611, 'dev2');
INSERT INTO github.users VALUES (535593, 'Boscop');
INSERT INTO github.users VALUES (13515498, 'Leroy-X');
INSERT INTO github.users VALUES (63473642, 'candemircan');
INSERT INTO github.users VALUES (25101888, 'alphastrata');
INSERT INTO github.users VALUES (182761, 'zaytsev');
INSERT INTO github.users VALUES (8382834, 'jerabaul29');
INSERT INTO github.users VALUES (44335182, 'kharf');
INSERT INTO github.users VALUES (66112470, 'BastienVigneron');
INSERT INTO github.users VALUES (68097476, 'thnguyen996');
INSERT INTO github.users VALUES (1308372, 'kiyo-e');
INSERT INTO github.users VALUES (53652902, 'haras-unicorn');
INSERT INTO github.users VALUES (28795812, 'fahdmirza');
INSERT INTO github.users VALUES (22633385, 'eltociear');
INSERT INTO github.users VALUES (32483567, 'Skipp1');
INSERT INTO github.users VALUES (94681915, 'noahfraiture');
INSERT INTO github.users VALUES (90320947, 'adriangalilea');
INSERT INTO github.users VALUES (15968876, 'a3lem');
INSERT INTO github.users VALUES (66897975, '4t8dd');
INSERT INTO github.users VALUES (55425467, 'Lunasciel');
INSERT INTO github.users VALUES (30024051, 'asukaminato0721');
INSERT INTO github.users VALUES (9112841, 'McPatate');
INSERT INTO github.users VALUES (8644490, 'RomeoV');
INSERT INTO github.users VALUES (28192100, 'cameroncarlg');
INSERT INTO github.users VALUES (2666479, 'SuperBo');
INSERT INTO github.users VALUES (162593633, 'patriote2047');
INSERT INTO github.users VALUES (2828428, 'andrejohansson');
INSERT INTO github.users VALUES (74125308, 'fxcl');
INSERT INTO github.users VALUES (109045880, 'Eyobs-droid');
INSERT INTO github.users VALUES (43348732, 'luixiao0');
INSERT INTO github.users VALUES (38045210, 'AgentElement');
INSERT INTO github.users VALUES (16382165, 'PrimeTimeTran');
INSERT INTO github.users VALUES (71929976, 'bajrangCoder');
INSERT INTO github.users VALUES (304428, 'Robzz');
INSERT INTO github.users VALUES (1927180, 'curldivergence');
INSERT INTO github.users VALUES (941359, 'sbromberger');
INSERT INTO github.users VALUES (108725, 'fsouza');
INSERT INTO github.users VALUES (68561, 'dirksierd');
INSERT INTO github.users VALUES (552829, 'transitive-bullshit');
INSERT INTO github.users VALUES (1445753, 'Nold360');
INSERT INTO github.users VALUES (837049, 'sayap');
INSERT INTO github.users VALUES (21687187, 'arunoruto');
INSERT INTO github.users VALUES (44469426, 'zoedsoupe');
INSERT INTO github.users VALUES (119697612, 'pierrelgol');
INSERT INTO github.users VALUES (6033387, 'peperunas');
INSERT INTO github.users VALUES (7916909, 'logankaser');
INSERT INTO github.users VALUES (147047, 'morganhein');
INSERT INTO github.users VALUES (479816, 'jokeyrhyme');
INSERT INTO github.users VALUES (50424412, 'Myzel394');
INSERT INTO github.users VALUES (71392160, 'AlejandroSuero');
INSERT INTO github.users VALUES (994357, 'fredrikaverpil');


--
-- Data for Name: repositories; Type: TABLE DATA; Schema: github; Owner: silas
--

INSERT INTO github.repositories VALUES (723094714, 'SilasMarvin/lsp-ai', 19626586, false, 'https://api.github.com/repos/SilasMarvin/lsp-ai');


--
-- Data for Name: app_installation_repositories; Type: TABLE DATA; Schema: github; Owner: silas
--

INSERT INTO github.app_installation_repositories VALUES (1, 723094714);

--
-- Data for Name: app_installations; Type: TABLE DATA; Schema: github; Owner: silas
--

INSERT INTO github.app_installations (installation_id, organization_id) VALUES (58242458, 1);


--
-- Data for Name: issues; Type: TABLE DATA; Schema: github; Owner: silas
--

INSERT INTO github.issues VALUES (2638136666, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/88', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/88/comments', 88, 'open', 'Emacs setup and config support', DEFAULT, 'Does anyone know how to integrate this with lsp-mode in Emacs? Emacs is listed as a compatible editor, and I have lsp-mode installed, but I’m unsure how to enable lsp-ai. Any help would be greatly appreciated!', DEFAULT, 1037150, '2024-11-06 13:43:08', '2024-11-06 13:43:08', NULL, NULL);
INSERT INTO github.issues VALUES (2622704997, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/87', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/87/comments', 87, 'open', 'Code 32, BrokenPipe in Helix', DEFAULT, 'Unable to make completions/code action completions work regardless of which configuration or example I use. In most cases lsp_ai returns error with broken pipe and code actions stop working as a result

Helix logs after successful sending of the data to lsp-ai:
```
helix_lsp::transport [ERROR] lsp-ai err: <- IO(Os { code: 32, kind: BrokenPipe, message: "Broken pipe" })
```

As a configuration I use example configuration from the documentation via ["--config", "path/to/json"]. Also tried extending the examples from examples directory, did not work either. The api key and model are working, checked with other tools.', DEFAULT, 44410570, '2024-10-30 02:31:32', '2024-10-30 02:31:32', NULL, NULL);
INSERT INTO github.issues VALUES (2588761718, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/86', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/86/comments', 86, 'closed', 'Wrong plasement for system message', DEFAULT, 'I am configuring code actions for helix with openai api. In the [provided instruction](https://github.com/SilasMarvin/lsp-ai/wiki/Configuration#custom-actions) system prompt is located as a model parameter:
```json
{
  "actions": [
    {
      "action_display_name": "Complete",
      "model": "model1",
      "parameters": {
        "max_context": 4096,
        "max_tokens": 4096,
        "system": "I am smart assistant...",
        "messages": [
          {
            "role": "user",
            "content": "{CODE}"
          }
        ]
      },
      "post_process": {
        "extractor": "(?s)<answer>(.*?)</answer>"
      }
    }
  ]
}

```

However, according to the [openai api docs](https://platform.openai.com/docs/api-reference/chat/create#chat-create-messages), system prompt should be placed at `messages`. Hence, correct version would look like:

```json
{
  "actions": [
    {
      "action_display_name": "Complete",
      "model": "model1",
      "parameters": {
        "max_context": 4096,
        "max_tokens": 4096,
        "messages": [
          {
            "role": "system",
            "content": "I am smart assistant..."
          },
          {
            "role": "user",
            "content": "{CODE}"
          }
        ]
      },
      "post_process": {
        "extractor": "(?s)<answer>(.*?)</answer>"
      }
    }
  ]
}

```', DEFAULT, 48477665, '2024-10-15 13:18:44', '2024-10-15 15:33:29', '2024-10-15 15:33:29', 19626586);
INSERT INTO github.issues VALUES (2582456515, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/85', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/85/comments', 85, 'open', 'Support Mistral Chat', DEFAULT, 'I see at the moment only the Mistral FIM is supported but Mistral Chat ( https://codestral.mistral.ai/v1/chat/completions ) is not supported.

I''m using custom actions with Helix so it would be useful to support Mistral Chat so custom instructions (prompt) can be passed in the parameters  (ex: To Refactor).

Thanks.', DEFAULT, 11238160, '2024-10-12 03:44:37', '2024-10-12 03:44:37', NULL, NULL);
INSERT INTO github.issues VALUES (2575475531, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/84', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/84/comments', 84, 'open', 'Is it possible to support GitHub Copilot Chat?', DEFAULT, 'As the titles says.

References:

[https://docs.github.com/en/copilot/quickstart](https://docs.github.com/en/copilot/quickstart)

[https://github.com/CopilotC-Nvim/CopilotChat.nvim](https://github.com/CopilotC-Nvim/CopilotChat.nvim)', DEFAULT, 13739892, '2024-10-09 10:19:50', '2024-11-27 00:09:52', NULL, NULL);
INSERT INTO github.issues VALUES (2569587938, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/83', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/83/comments', 83, 'open', 'support lazarus IDE ?', DEFAULT, NULL, DEFAULT, 34910769, '2024-10-07 07:26:50', '2024-10-07 07:26:50', NULL, NULL);
INSERT INTO github.issues VALUES (2563306159, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/82', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/82/comments', 82, 'closed', 'Helix code actions', DEFAULT, 'Hi there, 

Thanks for the tool, I''m attempting to set up this tool to work with Helix and enable code actions, but I''m encountering an issue where it reports that no code actions are available. Below is my configuration and a relevant log entry that might help diagnose the problem. 

Am I missing any necessary steps? Any guidance would be greatly appreciated! 

**Log**
```
2024-10-03T08:58:37.429 helix_lsp::transport [INFO] lsp-ai -> {"jsonrpc":"2.0","method":"textDocument/codeAction","params":{"context":{"diagnostics":[],"triggerKind":1},"range":{"end":{"character":1,"line":5},"start":{"character":0,"line":5}},"textDocument":{"uri":"file:///***********.py"}},"id":4}
2024-10-03T08:58:37.429 helix_lsp::transport [INFO] lsp-ai <- {"jsonrpc":"2.0","id":4,"result":[]}
2024-10-03T08:58:37.430 helix_lsp::transport [INFO] lsp-ai <- []
```

**Helix languages.toml**
```
[language-server.lsp-ai]
command = "lsp-ai"
args = ["--config", "*****/.config/helix/lsp-ai.json"]

[[language]]
name = "python"
language-servers = ["lsp-ai", "pylsp"]
```

**json**
```
{
  "initializationOptions": {
    "memory": {
      "file_store": {}
    },
    "models": {
      "model1": {
        "type": "open_ai",
        "chat_endpoint": "https://api.openai.com/v1/chat/completions",
        "model": "gpt-4o",
        "auth_token_env_var_name": "OPENAI_API_KEY"
      }
    }
  },
  "actions": [
    {
      "action_display_name": "Complete",
      "model": "model1",
      "parameters": {
        "max_context": 4096,
        "max_tokens": 4096,
        "system": "You are an AI coding assistant. Your task is to complete code snippets. The user''s cursor position is marked by \"<CURSOR>\". Follow these steps:\n\n1. Analyze the code context and the cursor position.\n2. Provide your chain of thought reasoning, wrapped in <reasoning> tags. Include thoughts about the cursor position, what needs to be completed, and any necessary formatting.\n3. Determine the appropriate code to complete the current thought, including finishing partial words or lines.\n4. Replace \"<CURSOR>\" with the necessary code, ensuring proper formatting and line breaks.\n5. Wrap your code solution in <answer> tags.\n\nYour response should always include both the reasoning and the answer. Pay special attention to completing partial words or lines before adding new lines of code.\n\n<examples>\n<example>\nUser input:\n--main.py--\n# A function that reads in user inpu<CURSOR>\n\nResponse:\n<reasoning>\n1. The cursor is positioned after \"inpu\" in a comment describing a function that reads user input.\n2. We need to complete the word \"input\" in the comment first.\n3. After completing the comment, we should add a new line before defining the function.\n4. The function should use Python''s built-in `input()` function to read user input.\n5. We''ll name the function descriptively and include a return statement.\n</reasoning>\n\n<answer>t\ndef read_user_input():\n    user_input = input(\"Enter your input: \")\n    return user_input\n</answer>\n</example>\n\n<example>\nUser input:\n--main.py--\ndef fibonacci(n):\n    if n <= 1:\n        return n\n    else:\n        re<CURSOR>\n\n\nResponse:\n<reasoning>\n1. The cursor is positioned after \"re\" in the ''else'' clause of a recursive Fibonacci function.\n2. We need to complete the return statement for the recursive case.\n3. The \"re\" already present likely stands for \"return\", so we''ll continue from there.\n4. The Fibonacci sequence is the sum of the two preceding numbers.\n5. We should return the sum of fibonacci(n-1) and fibonacci(n-2).\n</reasoning>\n\n<answer>turn fibonacci(n-1) + fibonacci(n-2)</answer>\n</example>\n</examples>",
        "messages": [
          {
            "role": "user",
            "content": "{CODE}"
          }
        ]
      },
      "post_process": {
        "extractor": "(?s)<answer>(.*?)</answer>"
      }
    },
    {
      "action_display_name": "Refactor",
      "model": "model1",
      "parameters": {
        "max_context": 4096,
        "max_tokens": 4096,
        "system": "You are an AI coding assistant specializing in code refactoring. Your task is to analyze the given code snippet and provide a refactored version. Follow these steps:\n\n1. Analyze the code context and structure.\n2. Identify areas for improvement, such as code efficiency, readability, or adherence to best practices.\n3. Provide your chain of thought reasoning, wrapped in <reasoning> tags. Include your analysis of the current code and explain your refactoring decisions.\n4. Rewrite the entire code snippet with your refactoring applied.\n5. Wrap your refactored code solution in <answer> tags.\n\nYour response should always include both the reasoning and the refactored code.\n\n<examples>\n<example>\nUser input:\ndef calculate_total(items):\n    total = 0\n    for item in items:\n        total = total + item[''price''] * item[''quantity'']\n    return total\n\n\nResponse:\n<reasoning>\n1. The function calculates the total cost of items based on price and quantity.\n2. We can improve readability and efficiency by:\n   a. Using a more descriptive variable name for the total.\n   b. Utilizing the sum() function with a generator expression.\n   c. Using augmented assignment (+=) if we keep the for loop.\n3. We''ll implement the sum() function approach for conciseness.\n4. We''ll add a type hint for better code documentation.\n</reasoning>\n<answer>\nfrom typing import List, Dict\n\ndef calculate_total(items: List[Dict[str, float]]) -> float:\n    return sum(item[''price''] * item[''quantity''] for item in items)\n</answer>\n</example>\n\n<example>\nUser input:\ndef is_prime(n):\n    if n < 2:\n        return False\n    for i in range(2, n):\n        if n % i == 0:\n            return False\n    return True\n\n\nResponse:\n<reasoning>\n1. This function checks if a number is prime, but it''s not efficient for large numbers.\n2. We can improve it by:\n   a. Adding an early return for 2, the only even prime number.\n   b. Checking only odd numbers up to the square root of n.\n   c. Using a more efficient range (start at 3, step by 2).\n3. We''ll also add a type hint for better documentation.\n4. The refactored version will be more efficient for larger numbers.\n</reasoning>\n<answer>\nimport math\n\ndef is_prime(n: int) -> bool:\n    if n < 2:\n        return False\n    if n == 2:\n        return True\n    if n % 2 == 0:\n        return False\n    \n    for i in range(3, int(math.sqrt(n)) + 1, 2):\n        if n % i == 0:\n            return False\n    return True\n</answer>\n</example>\n</examples>",
        "messages": [
          {
            "role": "user",
            "content": "{SELECTED_TEXT}"
          }
        ]
      },
      "post_process": {
        "extractor": "(?s)<answer>(.*?)</answer>"
      }
    }
  ]
}
```', DEFAULT, 1893361, '2024-10-03 07:16:40', '2024-10-30 02:16:48', '2024-10-11 07:38:27', 1893361);
INSERT INTO github.issues VALUES (2561734813, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/81', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/81/comments', 81, 'open', 'Unable to get completion with llama cpp', DEFAULT, 'Hello, I am trying to configure lsp-ai to get copilot-like completion in helix. I intend to use only models running locally. 
I ideally would like to get them served following an OpenAI compatible API, Llama.cpp provides this and so does mlx_lm server that I would like to use.

The problem is that I end up with a 400 error from the server that seems to receive a request under a wrong format. This happens **both** with llama cpp and mlx server so it doesn''t seem to be server-related, but a problem with lsp-ai. Is there a way to monitor the requests sent back/forth?

There''s a second option that I have tried, which is to use the direct llama_cpp feature from lsp-ai (how does it work? does it spawn it''s own separate instance of the server? What if we have one running already on the same ports, what about memory usage by an additional server if the models are big, compared to just linking to a running one through the openAI-compatible api?) 
Using the internal llama_cpp feature, it seems to send requests properly, at least the helix logs don''t show any error, but there is no completion displayed at all. 
Instead, here is what I am getting: (example on the right, note that the configuration shows on the left)
![image](https://github.com/user-attachments/assets/ac5dfc35-e120-4f2a-bcb9-7f5f59284fd0)
ai - text does nothing when selected.
Has anybody got any luck with this kind of configuration?

Edit Oct 12th
I have tried with Ollama following [this discussion](https://github.com/SilasMarvin/lsp-ai/issues/57) and got the same issue as with llama.cpp.  Besides, I tried to use visual studio with a similar configuration and got the same behavior.
It seems to be the way the request is sent that is problematic as it doesn''t seem to be built for `completion` (in that there is not really any next word to predict for the given prompt) but for `chat completion`, although it follows the `completion` standard format. 
For example: enabling verbose mode on llama.cpp, I can see the request sent being:

> request: POST /completions 127.0.0.1 200
> request:  {"echo":false,"frequency_penalty":0.0,"max_tokens":64,"model":"Qwen2.5-Coder-7B-Instruct-Q8_0","n":1,"presence_penalty":0.0,"prompt":"<fim_prefix>\n\ndef quicksort(arr):\n    if len(arr) <= 1:\n        return arr\n    pivot = arr[len<fim_suffix>\n\n    \n    middle = [x for x in arr if x == pivot]\n    right = [x for x in arr if x > pivot]\n    return quicksort(left) + middle + quicksort(right)\n<fim_middle>","temperature":0.10000000149011612,"top_p":0.949999988079071}

and the response (note the empty content)

> response: {"content":"","id_slot":0,"stop":true,"model":"../models/gguf/Qwen2.5-Coder-7B-Instruct-Q8_0.gguf","tokens_predicted":1,"tokens_evaluated":77,"generation_settings":{"n_ctx":4096,"n_predict":-1,"model":"../models/gguf/Qwen2.5-Coder-7B-Instruct-Q8_0.gguf","seed":4294967295,"seed_cur":979658403,"temperature":0.10000000149011612,"dynatemp_range":0.0,"dynatemp_exponent":1.0,"top_k":40,"top_p":0.949999988079071,"min_p":0.05000000074505806,"tfs_z":1.0,"typical_p":1.0,"repeat_last_n":64,"repeat_penalty":1.0,"presence_penalty":0.0,"frequency_penalty":0.0,"mirostat":0,"mirostat_tau":5.0,"mirostat_eta":0.10000000149011612,"penalize_nl":false,"stop":[],"max_tokens":64,"n_keep":0,"n_discard":0,"ignore_eos":false,"stream":false,"n_probs":0,"min_keep":0,"grammar":"","samplers":["top_k","tfs_z","typ_p","top_p","min_p","temperature"]},"prompt":"<fim_prefix>\n\ndef quicksort(arr):\n    if len(arr) <= 1:\n        return arr\n    pivot = arr[len<fim_suffix>\n\n    \n    middle = [x for x in arr if x == pivot]\n    right = [x for x in arr if x > pivot]\n    return quicksort(left) + middle + quicksort(right)\n<fim_middle>","truncated":false,"stopped_eos":true,"stopped_word":false,"stopped_limit":false,"stopping_word":"","tokens_cached":77,"timings":{"prompt_n":77,"prompt_ms":378.821,"prompt_per_token_ms":4.919753246753247,"prompt_per_second":203.2622267508929,"predicted_n":1,"predicted_ms":0.009,"predicted_per_token_ms":0.009,"predicted_per_second":111111.11111111112},"index":0}

The same request also returns an empty response content with curl, while, if sending a request (with the same prompt) under the `chat completion` format with curl:

> request: POST /v1/chat/completions 127.0.0.1 200
> request:  {
>      "messages": [{"role": "user", "content": "<fim_prefix>\\n\\ndef quicksort(arr):\\n    if len(arr) <= 1:\\n        return arr\\n    pivot = arr[len<fim_suffix>\\n\\n    \\n    middle = [x for x in arr if x == pivot]\\n    right = [x for x in arr if x > pivot]\\n    return quicksort(left) + middle + quicksort(right)\\n<fim_middle>"}],
>      "temperature": 0.7
>    }

The response seems to be spot on:

> response: {"choices":[{"finish_reason":"stop","index":0,"message":{"content":"```python\ndef quicksort(arr):\n    if len(arr) <= 1:\n        return arr\n    pivot = arr[len(arr) // 2]  # Choose the middle element as the pivot\n    left = [x for x in arr if x < pivot]  # Elements less than the pivot\n    middle = [x for x in arr if x == pivot]  # Elements equal to the pivot\n    right = [x for x in arr if x > pivot]  # Elements greater than the pivot\n    return quicksort(left) + middle + quicksort(right)\n```","role":"assistant"}}],"created":1728736275,"model":"gpt-3.5-turbo-0613","object":"chat.completion","usage":{"completion_tokens":123,"prompt_tokens":96,"total_tokens":219},"id":"chatcmpl-QXL7OiScMeo2SyWwAMdjR5DrWTXcqGBg","__verbose":{"content":"```python\ndef quicksort(arr):\n    if len(arr) <= 1:\n        return arr\n    pivot = arr[len(arr) // 2]  # Choose the middle element as the pivot\n    left = [x for x in arr if x < pivot]  # Elements less than the pivot\n    middle = [x for x in arr if x == pivot]  # Elements equal to the pivot\n    right = [x for x in arr if x > pivot]  # Elements greater than the pivot\n    return quicksort(left) + middle + quicksort(right)\n```","id_slot":0,"stop":true,"model":"gpt-3.5-turbo-0613","tokens_predicted":123,"tokens_evaluated":96,"generation_settings":{"n_ctx":4096,"n_predict":-1,"model":"../models/gguf/Qwen2.5-Coder-7B-Instruct-Q8_0.gguf","seed":4294967295,"seed_cur":2759634986,"temperature":0.699999988079071,"dynatemp_range":0.0,"dynatemp_exponent":1.0,"top_k":40,"top_p":0.949999988079071,"min_p":0.05000000074505806,"tfs_z":1.0,"typical_p":1.0,"repeat_last_n":64,"repeat_penalty":1.0,"presence_penalty":0.0,"frequency_penalty":0.0,"mirostat":0,"mirostat_tau":5.0,"mirostat_eta":0.10000000149011612,"penalize_nl":false,"stop":[],"max_tokens":-1,"n_keep":0,"n_discard":0,"ignore_eos":false,"stream":false,"n_probs":0,"min_keep":0,"grammar":"","samplers":["top_k","tfs_z","typ_p","top_p","min_p","temperature"]},"prompt":"<|im_start|>user\n<fim_prefix>\\n\\ndef quicksort(arr):\\n    if len(arr) <= 1:\\n        return arr\\n    pivot = arr[len<fim_suffix>\\n\\n    \\n    middle = [x for x in arr if x == pivot]\\n    right = [x for x in arr if x > pivot]\\n    return quicksort(left) + middle + quicksort(right)\\n<fim_middle><|im_end|>\n<|im_start|>assistant\n","truncated":false,"stopped_eos":true,"stopped_word":false,"stopped_limit":false,"stopping_word":"","tokens_cached":218,"timings":{"prompt_n":96,"prompt_ms":382.361,"prompt_per_token_ms":3.9829270833333332,"prompt_per_second":251.07163125946423,"predicted_n":123,"predicted_ms":3248.555,"predicted_per_token_ms":26.4110162601626,"predicted_per_second":37.86298831326543},"index":0,"oaicompat_token_ctr":123}}
> srv  add_waiting_: add task 142 to waiting list. current waiting = 0 (before add)

It could explain why it also fails with mlx and other OpenAI compatible API servers that all follow the same format. Haven''t been able to investigate why the direct llama.cpp type fails too, since I cannot control it''s logs and launch it in verbose mode, but I suspect the issue to be the same.', DEFAULT, 145156502, '2024-10-02 13:43:05', '2024-11-18 01:26:37', NULL, NULL);
INSERT INTO github.issues VALUES (2546591573, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/80', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/80/comments', 80, 'closed', 'Errors while running using nvim-lspconfig -- invalid length 0, expected struct FileStore with 1 element', DEFAULT, 'I am attempting to use `lsp-ai` with `nvim-lspconfig`, but I keep getting the following error whenever I enter a buffer with the correct file type, `lsp-ai` logs the following error:
```
2024-09-24T23:10:33.316442Z DEBUG lsp_server::msg: > {"jsonrpc":"2.0","id":1,"result":{"capabilities":{"codeActionProvider":{"resolveProvider":true},"completionProvider":{},"textDocumentSync":2}}}    
2024-09-24T23:10:33.371434Z DEBUG lsp_server::msg: < {"params":{},"jsonrpc":"2.0","method":"initialized"}    
2024-09-24T23:10:33.371473Z DEBUG lsp_server::stdio: sending message Notification(
    Notification {
        method: "initialized",
        params: Object {},
    },
)    
2024-09-24T23:10:33.371604Z ERROR lsp_ai: invalid length 0, expected struct FileStore with 1 element
```

But it seems the server is still running since it is still sending and receiving messages. I''ve tried using a local model with llama_cpp and using the "deepseek-coder" model from Ollama. Furthermore, it doesn''t look like Ollama is receiving the API calls (I''ve double-checked the port and everything).

I''m using nixvim (nixpkgs wrapper for neovim), though that should not matter too much since the relevant part of my config is written in raw lua.

LSP-AI log file: https://hastebin.com/share/tedozacagi.css


My `nvim-lspconfig` setup:
```lua
require(''lspconfig.configs'').lsp_ai = {
  default_config = {
    cmd = {
      ''${lsp-ai-llama-cpp}'',  # Use outPath from custom flake derivation (NixOS)
      ''--use-seperate-log-file'',
    },
    cmd_env = {
      LSP_AI_LOG = "DEBUG",
    },
    filetypes = { ''html'' },
    root_dir = vim.loop.cwd,
    init_options = {
      memory = {
        file_store = {}
      },
      models = {
        model1 = {
          type = "llama_cpp",
          file_path = "/home/user/Meta-Llama-3.1-8B-Instruct-F16.gguf",
          n_ctx = 2048,
          n_gpu_layers = 0,
        },
        model2 = {
          type = "ollama",
          model = "deepseek-coder",
        },
      },
      completion = {
        model = "model2",
        parameters = {
          fim = {
            start = "<|fim_prefix|>",
            middle = "<|fim_suffix|>",
            ["end"] = "<|fim_middle|>",
          },
          max_context = 2000,
          max_new_tokens = 32,
        }
      }
    },
  },
}

local capabilities = require(''cmp_nvim_lsp'').default_capabilities()

require(''lspconfig'').lsp_ai.setup ({
  capabilities = capabilities,
})
```', DEFAULT, 180627245, '2024-09-24 23:33:43', '2024-09-25 05:37:22', '2024-09-25 05:37:22', 19626586);
INSERT INTO github.issues VALUES (2545637874, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/79', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/79/comments', 79, 'closed', 'Fix tag issue / bump version', DEFAULT, NULL, DEFAULT, 19626586, '2024-09-24 14:51:21', '2024-09-24 14:51:29', '2024-09-24 14:51:29', 19626586);
INSERT INTO github.issues VALUES (2542699525, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/78', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/78/comments', 78, 'open', 'Can we make code editor using lsp-ai?', DEFAULT, 'Hello ! Can we make online code editor using lsp-ai? just like hacker rank and other online code editors ?', DEFAULT, 43817946, '2024-09-23 13:42:20', '2024-09-23 13:42:20', NULL, NULL);
INSERT INTO github.issues VALUES (2535072112, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/73', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/73/comments', 73, 'open', 'Idea: Crawl docs of dependencies + RAG', DEFAULT, 'I''m not sure if any of the available AI coding assistants do this (if you know, please tell me), but this is the main feature I''m missing:
Ideally it should crawl the docs of dependencies of the project and use this info when generating code.
This allows it to better respond to user requests because the user uses natural language that''s mostly found in documentation of symbols, not in the symbols themselves.
Too often these coding assistants hallucinate fake symbols because they were trained on outdated docs (e.g. ChatGPT and Claude always use outdated crate versions if you tell them to generate Rust code). 
If they could read the docs that wouldn''t happen :)
This feature would open the door to really low-code coding, just telling the LLM what to do and what to change!

In practice, how it would look like: E.g. lets say I have a Rust project with many deps and transitive deps.
I want to build a complex component using multiple symbols from different deps.
The assistant in the background inspects Cargo.toml to identify the deps, then crawls the docs.rs of the deps, adds these as vector embeddings etc. In addition, it also detects from the docs (or from rust-analyzer) which transitive deps are re-exported, and crawls their docs too. (Similar with other languages that have conventions around docs.)
Then, user prompts to the LLM via the LSP adapter use this contextual info, to generate code that uses the correct names/types/shapes/signatures.', DEFAULT, 535593, '2024-09-19 01:37:39', '2024-09-20 15:12:44', NULL, NULL);
INSERT INTO github.issues VALUES (2521844715, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/72', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/72/comments', 72, 'closed', 'How to use it in Kate editor?', DEFAULT, 'Hi, I am a newbie and unfamiliar with this. 
Does lsp-ai support Kate editor?  and how to use it in Kate?

Thanks.', DEFAULT, 13515498, '2024-09-12 09:23:35', '2024-09-16 01:45:21', '2024-09-12 14:32:44', 19626586);
INSERT INTO github.issues VALUES (2456334467, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/56', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/56/comments', 56, 'closed', 'crash on helix', DEFAULT, 'Hello,

Thank you for your initiative.
I''ve tried to configure helix as mentioned in the examples directory with : 

```toml
[language-server.lsp-ai.config.models.model1]
  type = "llama_cpp"
  repository = "stabilityai/stable-code-3b"
  name = "stable-code-3b-Q5_K_M.gguf"
  n_ctx = 2048

[language-server.lsp-ai.config.completion]
  model = "model1"

[language-server.lsp-ai.config.completion.parameters]
  max_tokens = 32
  max_context = 1024

[language-server.lsp-ai.config.completion.parameters.fim]
  start = "<fim_prefix>"
  middle = "<fim_suffix>"
  end = "<fim_middle>"

```

When I try to use it, I get the following error: 

```
2024-08-08T19:46:48.116 helix_lsp::transport [ERROR] Exiting lsp-ai after unexpected error: Parse(Error("data did not match any variant of untagged enum ServerMessage", line: 0, column: 0))
2024-08-08T19:46:50.389 helix_lsp::transport [ERROR] lsp-ai err <- "ERROR lsp_ai::transformer_worker: sending response: \"SendError(..)\"\n"
```

Any idea ?', DEFAULT, 66112470, '2024-08-08 17:52:55', '2024-08-13 01:14:58', '2024-08-13 01:14:57', 19626586);
INSERT INTO github.issues VALUES (2459766205, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/59', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/59/comments', 59, 'closed', 'Patched EOF error when chatting', DEFAULT, NULL, DEFAULT, 19626586, '2024-08-11 21:20:23', '2024-08-11 21:20:33', '2024-08-11 21:20:30', 19626586);
INSERT INTO github.issues VALUES (2456106129, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/54', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/54/comments', 54, 'closed', 'Patch Unicode Errors', DEFAULT, NULL, DEFAULT, 19626586, '2024-08-08 15:35:03', '2024-08-08 15:35:12', '2024-08-08 15:35:09', 19626586);
INSERT INTO github.issues VALUES (2449871303, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/48', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/48/comments', 48, 'closed', 'Updates to Crate dependencies and small features ', DEFAULT, NULL, DEFAULT, 19626586, '2024-08-06 03:43:10', '2024-08-06 03:43:20', '2024-08-06 03:43:17', 19626586);
INSERT INTO github.issues VALUES (2449858683, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/46', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/46/comments', 46, 'closed', 'Update README.md', DEFAULT, NULL, DEFAULT, 19626586, '2024-08-06 03:29:27', '2024-08-06 03:29:39', '2024-08-06 03:29:36', 19626586);
INSERT INTO github.issues VALUES (2379046522, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/38', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/38/comments', 38, 'closed', 'error: failed to compile `lsp-ai v0.3.0`', DEFAULT, 'Hello, I used the method that is recommended for Linux users with Nvidia GPUs and the installation failed.

I also tried `set CXX "g++-11"` before I ran the command, and it failed anyway


error: failed to compile `lsp-ai v0.3.0`, intermediate artifacts can be found at `/tmp/cargo-installIGWYNk`. To reuse those artifacts with a future compilation, set the environment variable `CARGO_TARGET_DIR` to that path.

Distro: Linux archlinux 6.9.6-1-cachyos-eevdf', DEFAULT, 55425467, '2024-06-27 20:34:06', '2024-08-08 14:04:41', '2024-08-08 14:04:41', 19626586);
INSERT INTO github.issues VALUES (2342192769, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/9', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/9/comments', 9, 'closed', '[Question] Is it possible to use Ollama as a backend?', DEFAULT, 'Hi, first of all - thank you for an awesome project :)
I have a question though - the docs say that when `llama.cpp` backend is used, the LSP links directly to it, but I''m not sure - is it possible to utilize an Ollama instance serving on the local network?
Thanks!', DEFAULT, 1927180, '2024-06-09 12:03:36', '2024-06-09 16:28:46', '2024-06-09 16:28:46', 19626586);
INSERT INTO github.issues VALUES (2542198034, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/77', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/77/comments', 77, 'closed', '''lsp-ai'' won''t be shown in ''hx --health''', DEFAULT, 'I''ve `added` this example to the file ''$HOME/.config/language.toml'', and then add ''lsp-ai'' to some languages as language server''
https://github.com/SilasMarvin/lsp-ai/blob/main/examples/helix/anthropic-in-editor-chatting.toml

But ''lsp-ai'' won''t be shown in ''hx --health''
Any required in addition?

os
ubuntu 24.04 in wsl2/windows11

helix version 
24.3

```
    Updating crates.io index
     Ignored package `lsp-ai v0.7.0` is already installed, use --force to override

$ which lsp-ai
/home/user/.cargo/bin/lsp-ai
```', DEFAULT, 40359643, '2024-09-23 10:23:24', '2024-09-23 10:48:28', '2024-09-23 10:48:28', 40359643);
INSERT INTO github.issues VALUES (2511293260, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/71', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/71/comments', 71, 'closed', 'serde error at installation', DEFAULT, 'Hi there,

I was trying to install the lsp using `cargo install lsp-ai`. However, I get the following error:

```
error: couldn''t read /Users/candemircan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.22/src/serde.rs: Operation not permitted (os error 1)
   --> /Users/candemircan/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.22/src/lib.rs:395:1
    |
395 | mod serde;
    | ^^^^^^^^^^

error: could not compile `log` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
error: failed to compile `lsp-ai v0.6.2`, intermediate artifacts can be found at `/var/folders/2t/zlktsbp913sc2d_5zx40s3rh0000gn/T/cargo-installINdsmm`.
To reuse those artifacts with a future compilation, set the environment variable `CARGO_TARGET_DIR` to that path.
```

I am using cargo 1.81.0 on an Apple Silicon machine. Many thanks for your help!
Best,
Can', DEFAULT, 63473642, '2024-09-06 22:55:15', '2024-09-20 15:13:32', '2024-09-20 15:13:31', 19626586);
INSERT INTO github.issues VALUES (2488389244, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/65', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/65/comments', 65, 'closed', 'Add binary to Github Release', DEFAULT, 'Many don''t want to use multiple package managers (cargo in this case) for maintaining packages.
Adding the precompiled binary to Github releases would make it easier to publish it to most common package managers.

', DEFAULT, 44335182, '2024-08-27 06:17:02', '2024-08-27 17:43:01', '2024-08-27 16:25:17', 19626586);
INSERT INTO github.issues VALUES (2468630958, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/60', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/60/comments', 60, 'open', 'Flake', DEFAULT, '# Description

Fixes #32

This PR adds shells and packages for the `cargo` workspace and `vscode` extension. The `nixos` and `home-manager` modules for the `lsp-ai` package were included because the issuer requested to add this easily to their `home-manager` configuration.

This is a WIP.

## Additional work to be done and requests for help

The build for the `lsp-ai` package fails because the build for `openssl-sys` fails and I haven''t been able to fix it. Any help around this would be greatly appreciated.

The `vscode` extension package needs to be modified to actually call `esbuild` but IDK what would be the right approach for that. On the other side, I don''t know if packaging it is even worthwhile because it already exists in the `vscode` marketplace and the [`vscode` extensions flake](https://github.com/nix-community/nix-vscode-extensions) most likely already picked it up so users can add it to their configuration through there.

## Possible regressions

`cargo2nix` modified the `Cargo.lock` file. If this is a problem I will try and make it so it doesn''t do that.

`node2nix` which the `vscode` extension flake uses requires the `package-lock.json` be downgraded to version 2 but if that is not acceptable i will remove that flake. The reason why that flake is added is to "nixify" the whole repository and there is surely a way to package up the `vscode` extension nicely so it can be added to `vscode` extensions `home-manager` configuration.

## Additional info

I use `direnv` for easily switching between environments in my shell so I added `.envrc` files. I hope this is not a problem.

Some additional lines were added to `.gitignore` to ease development which are optional and could be removed if it is a problem. The `.direnv` folder is only relevant if a developer uses `direnv` and `result` is the location where `nix build` saves the results of builds by default so unless someone is testing the result of `nix build` it is not needed.', DEFAULT, 53652902, '2024-08-15 18:09:41', '2024-08-16 15:53:20', NULL, NULL);
INSERT INTO github.issues VALUES (2451037524, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/49', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/49/comments', 49, 'closed', 'Update README.md', DEFAULT, NULL, DEFAULT, 19626586, '2024-08-06 14:29:20', '2024-08-06 14:29:30', '2024-08-06 14:29:27', 19626586);
INSERT INTO github.issues VALUES (2421318028, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/41', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/41/comments', 41, 'open', 'Post Process Extractors', DEFAULT, '[Edit because I pressed some random shortcut that submitted this before it was done]

Hi there. New to this tool and certainly not an LSP expert. I''ve been going through the prompt template examples, and it struck me as odd that nowhere is the LLM actually told what language it''s dealing with. Sure, it can probably infer it from the context, particularly if there are few-shot examples. But why make the LLM figure it out when we already know it.

I''ve been reading the LSP spec to see if that information is at any point provided to the LSP server by the client. It turns out [Document Synchronization](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_didOpen) messages contain an object with a `languageId` key, with values such as `python`. The logs of my editor (Helix) confirm that this key is sent by the client on message requests of this type.

I would use this key either by including it in the system prompt or by formatting code as Markdown blocks and adding it after the first set of triple backticks. An example of the second:

```json
{
      "role": "user",
      "content": "```{LANGUAGE}\ndef greet(name):\n    print(f\"Hello, {<CURSOR>}\")\n```"
}
```

What do you think? Is this something that could easily be added?', DEFAULT, 15968876, '2024-07-21 09:40:01', '2024-07-23 15:28:27', NULL, NULL);
INSERT INTO github.issues VALUES (2341547917, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/2', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/2/comments', 2, 'open', 'Neovim Plugin and nvim-lspconfig integration', DEFAULT, 'Hi,

I really would love to test this with neovim, but i have no idea how to setup a custom LSP. Maybe using `nvim-lspconfig`', DEFAULT, 1445753, '2024-06-08 08:24:51', '2024-06-28 01:40:38', NULL, NULL);
INSERT INTO github.issues VALUES (2539087774, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/76', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/76/comments', 76, 'closed', 'Release/v0.7.0', DEFAULT, NULL, DEFAULT, 19626586, '2024-09-20 15:22:32', '2024-09-24 14:51:49', '2024-09-20 15:22:41', 19626586);
INSERT INTO github.issues VALUES (2508058822, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/70', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/70/comments', 70, 'closed', 'Bump version', DEFAULT, NULL, DEFAULT, 19626586, '2024-09-05 15:04:22', '2024-09-05 15:04:46', '2024-09-05 15:04:46', 19626586);
INSERT INTO github.issues VALUES (2493688688, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/69', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/69/comments', 69, 'closed', 'Add support for external JSON config file', DEFAULT, 'New feature which adds support for loading configuration from a file: `lsp-ai --config ~/.config/lsp-ai.json`

Existing editor/plugin configs will be merged into the config read from file.

NB: to keep size of PR small i decided to keep compatibility with the current server `initializationOptions` so config should look like

```json
{
  "initializationOptions": {
    //actual config options
  }
}
```', DEFAULT, 182761, '2024-08-29 07:41:46', '2024-09-05 14:35:26', '2024-09-05 14:35:26', 19626586);
INSERT INTO github.issues VALUES (2479627865, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/64', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/64/comments', 64, 'closed', 'Error with neovim: expected struct FileStore with 1 element', DEFAULT, 'Hello, 
I''m having this error when setting up the server with neovim:
`ERROR lsp_ai: invalid length 0, expected struct FileStore with 1 element\n`

I''m using the native nvim lsp to configure the server. Using nvim-lspconfig resulted in the same error. 
Here is my configuration in the init.lua file: 

```sh
vim.lsp.start({
  name = ''lsp-ai'',
 cmd = ''path to lsp-ai binary'',
  root_dir = ''hard coded root dir just for testing'',
  init_options = {
  memory = {
  file_store = {},
      },
  model = {
  model1 = {
       type = "ollama", 
       model = "starcoder2-15b", 
      generate_endpoint = "http://.../generate", 
}
},
    completion = {
       model = "model1",
      parameters = {max_content = 300},
        } 
   } 
})
```
I believe it has something to do with the empty file_store option but I don''t really know how to fix that. 

', DEFAULT, 68097476, '2024-08-22 02:20:12', '2024-08-22 02:25:46', '2024-08-22 02:24:38', 19626586);
INSERT INTO github.issues VALUES (2453150358, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/53', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/53/comments', 53, 'open', 'provide some overviews of models / gpus to run these as a "tricks and tips"?', DEFAULT, 'Thanks for an amazing project!

One of the challenges when getting started with these llm lsps is getting a feeling for what is reasonable to run on different hardware.

Would it be within scope to have a dedicated documentation section with a few tables, recommending what model / config to use for different hardware? For example if I have on a CPU, what is a reasonable LLM to run? If I have a 4080 GPU, what is a good choice? I wonder if it feels a bit like a jungle otherwise for newcomers (at least it is my feeling now :) ).', DEFAULT, 8382834, '2024-08-07 10:34:41', '2024-08-07 14:16:24', NULL, NULL);
INSERT INTO github.issues VALUES (2449703999, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/45', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/45/comments', 45, 'closed', 'Add Chatting within your Editor', DEFAULT, NULL, DEFAULT, 19626586, '2024-08-06 00:41:39', '2024-08-06 02:56:49', '2024-08-06 02:56:43', 19626586);
INSERT INTO github.issues VALUES (2389880446, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/39', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/39/comments', 39, 'closed', 'Added a local vector store memory backend option', DEFAULT, NULL, DEFAULT, 19626586, '2024-07-04 03:52:48', '2024-07-27 18:51:42', '2024-07-27 18:46:42', 19626586);
INSERT INTO github.issues VALUES (2367561345, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/32', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/32/comments', 32, 'open', 'Add nix Flake', DEFAULT, 'Hey, this looks like it would be a great addition to the helix text editor! I was wondering if you know of any way to add this to a home-manager configuration file? Additionally, I have it downloaded but does not seem to run in any of my editors. 

Thanks!', DEFAULT, 28192100, '2024-06-22 05:02:06', '2024-08-08 14:05:45', NULL, NULL);
INSERT INTO github.issues VALUES (2354815766, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/24', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/24/comments', 24, 'closed', 'The code generated using the above prompt needs to be deleted before it can be used. Is there any good way to directly delete it?', DEFAULT, '```
"lsp-ai.serverConfiguration": {
        "memory": {
            "file_store": {}
          },
          "models": {
            "model1": {
              "type": "open_ai",
              "chat_endpoint": "https://api.openai.com/v1/chat/completions",
              "model": "gpt-4o",
              "auth_token": "sk-xxxxxxxxxxxxxx"
            }
          },
          "completion": {
            "model": "model1",
            "parameters": {
              "max_context": 2048,
              "max_tokens": 128,
              "messages": [
                {
                  "role": "system",
                  "content": "Instructions:\n- You are an AI programming assistant.\n- Given a piece of code with the cursor location marked by \"<CURSOR>\", replace \"<CURSOR>\" with the correct code or comment.\n- First, think step-by-step.\n- Describe your plan for what to build in pseudocode, written out in great detail.\n- Then output the code replacing the \"<CURSOR>\"\n- Ensure that your completion fits within the language context of the provided code snippet (e.g., Python, JavaScript, Rust).\n\nRules:\n- Only respond with code or comments.\n- Only replace \"<CURSOR>\"; do not include any previously written code.\n- Never include \"<CURSOR>\" in your response\n- If the cursor is within a comment, complete the comment meaningfully.\n- Handle ambiguous cases by providing the most contextually appropriate completion.\n- Be consistent with your responses."
                },
                {
                  "role": "user",
                  "content": "def greet(name):\n    print(f\"Hello, {<CURSOR>}\")"
                },
                {
                  "role": "assistant",
                  "content": "name"
                },
                {
                  "role": "user",
                  "content": "function sum(a, b) {\n    return a + <CURSOR>;\n}"
                },
                {
                  "role": "assistant",
                  "content": "b"
                },
                {
                  "role": "user",
                  "content": "fn multiply(a: i32, b: i32) -> i32 {\n    a * <CURSOR>\n}"
                },
                {
                  "role": "assistant",
                  "content": "b"
                },
                {
                  "role": "user",
                  "content": "# <CURSOR>\ndef add(a, b):\n    return a + b"
                },
                {
                  "role": "assistant",
                  "content": "Adds two numbers"
                },
                {
                  "role": "user",
                  "content": "# This function checks if a number is even\n<CURSOR>"
                },
                {
                  "role": "assistant",
                  "content": "def is_even(n):\n    return n % 2 == 0"
                },
                {
                  "role": "user",
                  "content": "{CODE}"
                }
              ]
            }
          }
    },
    ```
The code generated using the above prompt needs to be deleted before it can be used.
Is there any good way to directly delete it?


def TreeNode:```python
    # Binary tree node definition
    def __init__(self, value=0, left=None, right=None):
        self.value = value
        self.left = left
        self.right = right
```python
# Function to create a binary tree from a list
def create_binary_tree(lst, index=0):
    if index < len(lst):
        node = TreeNode(lst[index])
        node.left = create_binary_tree(lst, 2 * index + 1)
        node.right = create_binary_tree(lst, 2 * index + 2)
        return node
    return None
```python
# Example usage
if __name__ == "__main__":
    elements = [1, 2, 3, 4, 5, 6, 7]
    root = create_binary_tree(elements)
    print(root.value)  # Output: 1
```python
```python
print(root.left.value)  # Output: 2
```print(root.right.value)  # Output: 3

```

', DEFAULT, 74125308, '2024-06-15 12:21:53', '2024-06-16 22:49:33', '2024-06-16 22:49:33', 19626586);
INSERT INTO github.issues VALUES (2342327300, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/10', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/10/comments', 10, 'closed', 'Added  file_path config option for llama_cpp models', DEFAULT, NULL, DEFAULT, 19626586, '2024-06-09 15:06:37', '2024-06-10 14:34:32', '2024-06-09 15:06:47', 19626586);
INSERT INTO github.issues VALUES (2342112427, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/7', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/7/comments', 7, 'closed', 'Fixed Anthropic API by creating OpenAIChatMessage', DEFAULT, NULL, DEFAULT, 19626586, '2024-06-09 08:58:44', '2024-06-10 14:39:55', '2024-06-09 08:59:32', 19626586);
INSERT INTO github.issues VALUES (2204079308, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/1', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/1/comments', 1, 'closed', 'Silas async overhaul', DEFAULT, NULL, DEFAULT, 19626586, '2024-03-23 22:47:39', '2024-05-28 23:35:13', '2024-03-24 02:02:47', 19626586);
INSERT INTO github.issues VALUES (2538911233, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/75', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/75/comments', 75, 'closed', 'Update Cargo.lock', DEFAULT, 'lsp-ai version did not get bumped in lock file. Breaks CI checks that look for inconsistencies between toml file and lock files.

Should be merged into releases and mainline so systems that build against your release branch will also get updated. ', DEFAULT, 6314611, '2024-09-20 14:01:14', '2024-09-20 15:22:01', '2024-09-20 15:22:00', 19626586);
INSERT INTO github.issues VALUES (2489825675, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/66', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/66/comments', 66, 'closed', 'Release/v0.6.2', DEFAULT, NULL, DEFAULT, 19626586, '2024-08-27 16:13:52', '2024-08-27 16:14:02', '2024-08-27 16:13:59', 19626586);
INSERT INTO github.issues VALUES (2471642512, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/61', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/61/comments', 61, 'closed', 'Custom actions, server shutdown fixes and a bunch of small things', DEFAULT, NULL, DEFAULT, 19626586, '2024-08-17 19:30:56', '2024-08-21 02:55:58', '2024-08-21 02:55:54', 19626586);
INSERT INTO github.issues VALUES (2451215027, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/50', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/50/comments', 50, 'closed', 'Panic when seeing unicode', DEFAULT, 'lsp-ai seems to panic when there is unicode in the source code. 

```
thread ''tokio-runtime-worker'' panicked at /home/user/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lsp-ai-0.4.0/src/transformer_worker.rs:122:60:
byte index 25 is not a char boundary; it is inside ''μ'' (bytes 24..26) of `plt.xlabel(''Wavelength (μm)'')`
```
## Version:
lsp-ai v0.4.0
Opensuse Linux 6.10.2-1-default
rustc 1.79.0

## Config: 
```json
"initializationOptions": {
    "models": {
        "model1": {
            "type": "open_ai",
            "chat_endpoint": "https://api.groq.com/openai/v1/chat/completions",
            "model": "llama3-70b-8192",
            "auth_token": "<HIDDEN>"
        }
    }, 
    "completion": {
        "model": "model1",
        "parameters": {
            "max_context": 2048,
            "max_tokens": 128, 
            "messages": [{
               "role": "system",
               "content": "You are a programming completion tool. Replace <CURSOR> with the correct code."
            },{
                "role": "user",
                "content": "{CODE}"
            }]
        }
    }, 
    "memory": {
        "file_store": {}
    }
}
```', DEFAULT, 32483567, '2024-08-06 15:53:26', '2024-08-08 15:39:18', '2024-08-08 15:39:18', 19626586);
INSERT INTO github.issues VALUES (2426171444, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/42', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/42/comments', 42, 'open', '[FEATURE REQUEST] Add Copilot', DEFAULT, 'As seen in [helix-gpt](https://github.com/leona/helix-gpt)', DEFAULT, 90320947, '2024-07-23 21:36:06', '2024-08-06 19:24:37', NULL, NULL);
INSERT INTO github.issues VALUES (2373729518, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/34', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/34/comments', 34, 'open', 'crazy idea: enhance onhover for function', DEFAULT, 'when coding with llm, we often ask llm what''s one function/one part of code''s functionality.

but traditional lsp could only show comment and function signatures.

so if onhover could also query the functionality, will be useful.

pros: more info from llm.

cons: increase the cost, the ai will generate different answers each time, the respond speed, etc.
', DEFAULT, 30024051, '2024-06-25 21:53:42', '2024-06-26 23:40:05', NULL, NULL);
INSERT INTO github.issues VALUES (2354890213, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/26', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/26/comments', 26, 'closed', 'Problème d''installation sur Windows 11 :  "cargo install lsp-ai -F llama_cpp"', DEFAULT, 'En regardant le Tuto de : [AICodeKing](https://www.youtube.com/watch?v=f9BuYU9wjMA) pour installer **LSP-AI** pour **Ollama** sur Windows 11.
Je me suis retrouvé avec ces erreurs : 
![CARGO-PROJETS - Visual Studio Code](https://github.com/SilasMarvin/lsp-ai/assets/162593633/c17029dc-6b73-4d26-ade8-ea2c5fcfc609)
J''ai donc un peu cherché et j''ai trouvé ! 

🥇 🥇 🥇 INSTALLER sur Windows 11 & Python 3.12 & Conda 🥇 🥇 🥇 

**PRE-INSTALLATION SYSTEME :** 
[Python](https://www.python.org/downloads/)
[MiniConda](https://docs.anaconda.com/free/miniconda/)
[Installer Microsoft C++ Build Tools](https://aka.ms/vs/17/release/vs_BuildTools.exe)
[Installer Rust](https://www.rust-lang.org/tools/install)
[Installer Visual Studio Code](https://code.visualstudio.com/docs/?dv=win)
[Installer LLVM 18.1.5](https://github.com/llvm/llvm-project/releases/download/llvmorg-18.1.5/LLVM-18.1.5-win64.exe) 💯  DECLARER : **COUPABLE**
>>>>Rajouter les variables ENV au système pour llama.cpp.

**INSTALLATION de LSP-AI :***
Créer un ENV conda & l''activer.
cloner le Depot.
Se déplacer dans le dossier créé.
Vérifier que Cargo est bien installer. (Redémarrer si il y a un probléme)
Lancer la commande d''installation. 
```
### Créer un ENV conda & l''activer.
conda create -n lsp-ai python=3.12 -y
conda activate lsp-ai

### cloner le Depot.
git clone https://github.com/SilasMarvin/lsp-ai.git

### Se déplacer dans le dossier créé.
cd lsp-ai

### Vérifier que Cargo est bien installer.
cargo --version 

###### Si OK ! alors on continu...
### Lancer la commande d''installation. 
cargo install lsp-ai -F llama_cpp

# Installation Terminée.

```
### Nouvelle Réponse 👍 
![toml - CARGO-PROJETS - Visual Studio Code](https://github.com/SilasMarvin/lsp-ai/assets/162593633/daafc7fb-2a47-438e-a153-6dedba2b63b4)
👍 👍 👍 
Bonne Installation...

### Installer LSP-AI pour VScode
[LSP-AI pour VScode](https://marketplace.visualstudio.com/items?itemName=SilasMarvin.lsp-ai)
[Ollama](https://ollama.com/)

_Ps: l''installation a été faite avec un .env dans ''VisualStudio" mais il n''y ne devrait pas y avoir de soucis avec conda._

### Bonus :

1. [Le Plugin dans VScode.](https://github.com/SilasMarvin/lsp-ai/wiki/Plugins)
2. [Configuration pour Ollama.](https://github.com/SilasMarvin/lsp-ai/wiki/Configuration#models)


', DEFAULT, 162593633, '2024-06-15 14:27:25', '2024-08-08 14:09:38', '2024-08-08 14:09:38', 19626586);
INSERT INTO github.issues VALUES (2343344751, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/15', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/15/comments', 15, 'closed', 'support api_endpoint  for ollama', DEFAULT, NULL, DEFAULT, 43348732, '2024-06-10 09:11:07', '2024-06-12 23:58:13', '2024-06-12 23:58:12', 19626586);
INSERT INTO github.issues VALUES (2341844712, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/5', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/5/comments', 5, 'open', 'Support for `textDocument/inlineCompletion`?', DEFAULT, 'I realize that this is still in the upcoming version of the LSP spec and not supported by editors yet, but curious if you''re planning to support [textDocument/inlineCompletion](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.18/specification/#textDocument_inlineCompletion)?

Thanks for your work on this!', DEFAULT, 108725, '2024-06-08 20:56:08', '2024-06-08 23:38:22', NULL, NULL);
INSERT INTO github.issues VALUES (2537650065, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/74', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/74/comments', 74, 'open', 'Simplified nix flake for build, dev shell, and cached build', DEFAULT, 'I see #60. Honestly not sure which method is better at the moment. This PR allows local building with `nix build` and a development shell with `nix develop`. Ideally adding an entry to nixpkgs might solve #60 and #32 since it will be built into the eco-system. Open to discussion. Still learning nix myself.', DEFAULT, 6314611, '2024-09-20 01:40:05', '2024-11-15 15:41:09', NULL, NULL);
INSERT INTO github.issues VALUES (2491879042, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/67', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/67/comments', 67, 'closed', 'running the LSP on a remote machine / using a remote LLM LSP server: documentation / if necessary general implementation (?)', DEFAULT, 'Many thanks again for an awesome tool :) .

If I understand correctly, it is possible to run the LSP on a remote machine (for example running the LSP on a GPU server, while coding on a laptop, at least for a specific LLM backend https://github.com/SilasMarvin/lsp-ai/pull/15 ).

This is actually a common user case I think - for example at companies, it would be much more convenient to have a shared GPU server running the LLMs, and employees connecting their LSP editor client to it, rather than each employee getting a GPU and running a LLM server on their own machine.

It is not clear to me if the use of a remote server to actually run the LLM is only supported for a specific model after the PR above, or if it is a general feature. If it was possible to support it as a general feature, this would be very convenient :) . Not sure what the architecture of lsp-ai is at now, and not sure for example if the server vs "client" part of the LSP are decoupled and if this would be as simple as ssh-piping from a machine running the LLM to the local machine on a given port. Could this be documented / explained as an example of deployment? :)

This would have a lot to say for adoption, since I am afraid at present it is not realistic to hope that each developer would have a powerful enough GPU in their machine running a LLM, and without such hardware, the models that can be run are very small due to both latency and overall memory use considerations :) .', DEFAULT, 8382834, '2024-08-28 11:57:29', '2024-08-28 14:03:29', '2024-08-28 13:59:46', 19626586);
INSERT INTO github.issues VALUES (2478355341, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/62', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/62/comments', 62, 'closed', 'An error occurred with the VSCode extension.', DEFAULT, 'An error occurred with the VSCode extension.
Upon checking the logs, it says:
`error: unexpected argument ''--stdio'' found`

Here is the full error message:
```
error: unexpected argument ''--stdio'' found

Usage: lsp-ai [OPTIONS]

For more information, try ''--help''.
[Error - 0:27:23] Server process exited with code 2.
[Error - 0:27:23] Server initialization failed.
  Message: Pending response rejected since connection got disposed
  Code: -32097 
[Info  - 0:27:23] Connection to server got closed. Server will restart.
true
[Error - 0:27:23] lsp-ai client: couldn''t create connection to server.
  Message: Pending response rejected since connection got disposed
  Code: -32097 
```

versions:
lsp-ai 0.6.0
VSCode extension: 0.1.0 
', DEFAULT, 1308372, '2024-08-21 15:29:55', '2024-08-21 15:41:31', '2024-08-21 15:41:31', 19626586);
INSERT INTO github.issues VALUES (2451504473, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/51', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/51/comments', 51, 'closed', 'chore: update config.rs', DEFAULT, 'specfied -> specified', DEFAULT, 22633385, '2024-08-06 18:59:09', '2024-08-06 19:42:27', '2024-08-06 19:42:27', 19626586);
INSERT INTO github.issues VALUES (2375935712, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/36', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/36/comments', 36, 'open', 'refactor: error handling', DEFAULT, NULL, DEFAULT, 9112841, '2024-06-26 17:47:35', '2024-07-11 17:09:50', NULL, NULL);
INSERT INTO github.issues VALUES (2366824348, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/31', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/31/comments', 31, 'closed', 'Introduce RAG', DEFAULT, NULL, DEFAULT, 19626586, '2024-06-21 15:57:41', '2024-06-25 04:26:28', '2024-06-25 04:26:26', 19626586);
INSERT INTO github.issues VALUES (2355576107, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/27', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/27/comments', 27, 'closed', 'Allow to specify local model when using `llama_cpp`', DEFAULT, 'For now, LSP-AI will try to download model file when config with `llama_cpp`. Are there anyway for user to specify to an already downloaded model in local folder?', DEFAULT, 2666479, '2024-06-16 08:50:52', '2024-06-16 14:43:54', '2024-06-16 14:43:54', 19626586);
INSERT INTO github.issues VALUES (2346563469, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/21', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/21/comments', 21, 'closed', 'VS code ', DEFAULT, '[Error - 5:16:50 PM] lsp-ai client: couldn''t create connection to server.
Launching server using command lsp-ai failed. Error: spawn lsp-ai ENOENT

The following error shows on vs code insiders', DEFAULT, 109045880, '2024-06-11 14:18:41', '2024-06-11 15:49:03', '2024-06-11 15:49:03', 19626586);
INSERT INTO github.issues VALUES (2342369376, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/11', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/11/comments', 11, 'closed', 'Added Ollama as a backend', DEFAULT, NULL, DEFAULT, 19626586, '2024-06-09 16:28:08', '2024-06-10 14:17:59', '2024-06-09 16:28:18', 19626586);
INSERT INTO github.issues VALUES (2341859996, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/6', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/6/comments', 6, 'closed', 'Sample helix languages.toml?', DEFAULT, 'I am having difficulty figuring out how to pass `initializationOptions` via helix. Any way you might be able to provide a quick helix config as an example? Thanks.', DEFAULT, 941359, '2024-06-08 21:37:08', '2024-06-08 21:37:46', '2024-06-08 21:37:46', 941359);
INSERT INTO github.issues VALUES (2459757335, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/58', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/58/comments', 58, 'closed', 'Add custom log file and improve logging', DEFAULT, NULL, DEFAULT, 19626586, '2024-08-11 20:53:25', '2024-08-11 20:53:45', '2024-08-11 20:53:42', 19626586);
INSERT INTO github.issues VALUES (2492322529, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/68', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/68/comments', 68, 'closed', 'Add support for external config file', DEFAULT, 'It would be nice to have ability to configure lsp-ai with external file, instead of embedding config into helix `languages.toml`

Context:
I created module for [home-manager](https://github.com/nix-community/home-manager) to have a sort of type/syntax checked config generator and only then found out that there is no direct way to feed generated JSON into lsp-ai
', DEFAULT, 182761, '2024-08-28 14:46:55', '2024-09-05 14:35:27', '2024-09-05 14:35:27', 19626586);
INSERT INTO github.issues VALUES (2478376063, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/63', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/63/comments', 63, 'closed', 'Patch stdio error with vscode', DEFAULT, 'Add `stdio` as a dummy command line argument for now.', DEFAULT, 19626586, '2024-08-21 15:40:13', '2024-08-21 15:40:25', '2024-08-21 15:40:22', 19626586);
INSERT INTO github.issues VALUES (2451684749, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/52', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/52/comments', 52, 'open', 'Compilation Error on Ubuntu 22.04', DEFAULT, 'Hello,
    I am trying to install this lsp-ai on Ubuntu 22.04 and getting following error:

  Compiling atoi v2.0.0
error: Unrecognized option: ''diagnostic-width''

error: could not compile `atoi` (lib)
warning: build failed, waiting for other jobs to finish...
error: failed to compile `lsp-ai v0.4.0`, intermediate artifacts can be found at `/tmp/cargo-installbmlwJ1`.
To reuse those artifacts with a future compilation, set the environment variable `CARGO_TARGET_DIR` to that path.

(lsp) Ubuntu@0136-ict-prxmx50056:~/lsp-ai$ rustc -V
rustc 1.80.0 (051478957 2024-07-21)
(lsp) Ubuntu@0136-ict-prxmx50056:~/lsp-ai$ cargo --version
cargo 1.75.0

Also , its not clear from documentation what to do after you get lsp-ai installed? How do you integrate it with VSCode. After going through existing issues, I managed to get that lsp-ai extension needed to be installed. Once that extension is installed , how do I configure it to use lsp-ai server and Ollama? Where do I put that Ollama configuration which you have pasted in wiki?

Could I please request you to clarify above points as its not clear from documentation. Thanks. 

', DEFAULT, 28795812, '2024-08-06 20:59:11', '2024-08-07 05:06:49', NULL, NULL);
INSERT INTO github.issues VALUES (2442606878, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/44', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/44/comments', 44, 'closed', 'Pull Ollama Models', DEFAULT, 'When running Ollama models we need to pull them first in case the user hasn''t already or they will get an error saying they need to pull.', DEFAULT, 19626586, '2024-08-01 14:19:52', '2024-08-17 19:42:10', '2024-08-17 19:42:10', 19626586);
INSERT INTO github.issues VALUES (2434119177, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/43', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/43/comments', 43, 'closed', 'User help with Helix', DEFAULT, 'Hello, 
I''m trying to get things work but it doesn''t and I don''t know why. I used the configuration here [https://github.com/SilasMarvin/lsp-ai/blob/main/examples/helix/openai-chat.toml](url) in rust and I simply changed 
```
chat_endpoint = "https://api.groq.com/openai/v1/chat/completions"
auth_token_env_var_name = "GROQ_API_KEY"
```

I''m mostly here to ask if there''s any logs to debug that
', DEFAULT, 94681915, '2024-07-28 21:23:00', '2024-07-29 13:44:06', '2024-07-29 13:44:06', 94681915);
INSERT INTO github.issues VALUES (2374752627, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/35', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/35/comments', 35, 'closed', 'feat: add release CI', DEFAULT, 'Ported the CI from llm-ls', DEFAULT, 9112841, '2024-06-26 08:49:54', '2024-06-27 06:06:52', '2024-06-27 06:06:48', 19626586);
INSERT INTO github.issues VALUES (2355993542, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/28', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/28/comments', 28, 'closed', 'Add google generative language api support', DEFAULT, 'https://aistudio.google.com/app/prompts/new_chat

currently it''s free.

usage

```sh
curl \
  -H ''Content-Type: application/json'' \
  -d ''{"contents":[{"parts":[{"text":"Explain how AI works"}]}]}'' \
  -X POST ''https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key=YOUR_API_KEY''
```', DEFAULT, 30024051, '2024-06-16 21:35:01', '2024-06-21 15:15:57', '2024-06-21 15:15:57', 19626586);
INSERT INTO github.issues VALUES (2346771112, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/22', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/22/comments', 22, 'closed', 'Updated Cargo.lock', DEFAULT, NULL, DEFAULT, 19626586, '2024-06-11 15:51:11', '2024-06-11 15:52:12', '2024-06-11 15:52:07', 19626586);
INSERT INTO github.issues VALUES (2342373064, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/12', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/12/comments', 12, 'closed', 'Bumped crate version for 0.2.0', DEFAULT, NULL, DEFAULT, 19626586, '2024-06-09 16:37:27', '2024-06-09 16:37:39', '2024-06-09 16:37:34', 19626586);
INSERT INTO github.issues VALUES (2458890504, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/57', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/57/comments', 57, 'open', 'Documentation request: Ollama + Helix from scratch ', DEFAULT, 'So, the existing example/xyz.tomls aren''t exactly easy to get going in the helix philosophy of ''batteries included".

I think we have two issues:

1) Perhaps suggesting that users get started with a local `.helix/languages.toml` which if alongside their source will get merged with their system''s `languages.toml` as defined by wherever they set the `$HELIX_RUNTIME` to. This will be especially problematic to debug as, whatever languge they choose to add `lsp-ai` to as an additional lsp will really blast their `$HELIX_LOG` (as it will at a minimum have multiple lsp''s output in there)

2) Having a bind setup for triggering autocomplete (or chat?) will involve them adding things to their regular `config.toml`, you cannot (at least on latest helix) make a keybinding in a `languages.toml`.

Perhaps it''d be worth (rather than providing dozens of snippets for supported models in json across the [configuration](https://github.com/SilasMarvin/lsp-ai/wiki/Configuration) in favour of one or two thorough examples that take users from soup to nuts.

As this is somewhat positioned as a copilot alternative, I worry that the project will miss out on a great deal of users and such as a result of the amount of understanding required to get this tol running.

Hope you keep it up, this is a great idea!

', DEFAULT, 25101888, '2024-08-10 04:24:23', '2024-12-10 18:50:22', NULL, NULL);
INSERT INTO github.issues VALUES (2456111460, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/55', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/55/comments', 55, 'closed', 'Bump version', DEFAULT, NULL, DEFAULT, 19626586, '2024-08-08 15:37:48', '2024-08-08 15:38:05', '2024-08-08 15:37:55', 19626586);
INSERT INTO github.issues VALUES (2449859817, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/47', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/47/comments', 47, 'closed', 'Bumped version', DEFAULT, NULL, DEFAULT, 19626586, '2024-08-06 03:30:50', '2024-08-06 03:31:29', '2024-08-06 03:30:56', 19626586);
INSERT INTO github.issues VALUES (2420738486, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/40', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/40/comments', 40, 'closed', 'bug: lsp_ai::transformer_worker: generating response: specify `completions_endpoint` to use completions. Wanted to use `chat` instead?', DEFAULT, 'My Os is MacOS, M2. 

The config of vscode is:
```
{
  "lsp-ai.serverConfiguration": {
    "memory": {
      "file_store": {}
    },
    "models": {
      "model1": {
        "type": "open_ai",
        "chat_endpoint": "https://api.openai.com/v1/chat/completions",
        "model": "gpt-4o",
        "auth_token": "sk-codefB2"
      }
    }
  },
  "lsp-ai.generationConfiguration": {
    "model": "model1",
    "parameters": {
      "max_tokens": 128,
      "max_context": 1024,
      "messages": [
        {
          "role": "system",
          "content": "You are a programming completion tool. Replace <CURSOR> with the correct code."
        },
        {
          "role": "user",
          "content": "{CODE}"
        }
      ]
    }
  },
  "lsp-ai.inlineCompletionConfiguration": {
    "maxCompletionsPerSecond": 1
  }
}


```
The error I got is:

ERROR lsp_ai::memory_worker: error in memory worker task: Error file not found
ERROR dispatch_request{request=Generation(GenerationRequest { id: RequestId(I32(124)), params: GenerationParams { text_document_position: TextDocumentPositionParams { text_document: TextDocumentIdentifier { uri: Url { scheme: "file", cannot_be_a_base: false, username: "", password: None, host: None, port: None, path: "/Users/mme/repos/fib.py", query: None, fragment: None } }, position: Position { line: 1, character: 14 } }, model: "model1", parameters: Object {"max_context": Number(2048), "max_tokens": Number(128)}, post_process: PostProcess { remove_duplicate_start: true, remove_duplicate_end: true } } })}: lsp_ai::transformer_worker: generating response: specify `completions_endpoint` to use completions. Wanted to use `chat` instead? Please specify `chat_endpoint` and `messages`.
DEBUG lsp_server::msg: > {"jsonrpc":"2.0","id":124,"error":{"code":-32603,"message":"specify `completions_endpoint` to use completions. Wanted to use `chat` instead? Please specify `chat_endpoint` and `messages`."}}
```

Some issue in my config? Or I missed anything. Please help me out before I have to read the rust code.', DEFAULT, 66897975, '2024-07-20 08:48:52', '2024-07-20 10:42:52', '2024-07-20 10:42:16', 66897975);
INSERT INTO github.issues VALUES (2345456055, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/19', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/19/comments', 19, 'open', 'Add doc for printing LSP server logs', DEFAULT, '![demo](https://github.com/SilasMarvin/lsp-ai/assets/16382165/e579a9ff-9dd8-4a25-8d5d-55c057c2337a)
I''m working on getting a handle on logging so implement features like list suggestions, next suggestion,  select suggestions, etc.

I''ve got LSP-AI working on my local & can start the extension and generate a response as well as confirm the generation.

I set `LSP_AI_LOG=DEBUG` before running app.

However I haven''t gotten a handle on how to print LSP server logs. 

Can we add doc explaining that?
', DEFAULT, 16382165, '2024-06-11 05:52:38', '2024-06-11 16:00:10', NULL, NULL);
INSERT INTO github.issues VALUES (2343367809, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/16', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/16/comments', 16, 'closed', 'Support remote endpoint for Ollama?', DEFAULT, 'Thanks for this project

### Is there an existing issue for this?
   I have searched the existing issues

### Feature request
   Support remote endpoint for Ollama

### Context
   ollama is not fast enough using onboard hardware if run on laptops locally, we can utilize exising cloud GPUs to speed it up

Possible implementation
  I have provided a possible implementation at https://github.com/SilasMarvin/lsp-ai/pull/15', DEFAULT, 43348732, '2024-06-10 09:17:45', '2024-06-13 02:55:14', '2024-06-13 02:55:13', 43348732);
INSERT INTO github.issues VALUES (2342658847, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/14', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/14/comments', 14, 'closed', 'RAG', DEFAULT, 'This PR adds semantic search as a method for retrieving context. ', DEFAULT, 19626586, '2024-06-10 01:40:55', '2024-06-25 04:26:41', '2024-06-21 15:57:49', 19626586);
INSERT INTO github.issues VALUES (2376975741, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/37', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/37/comments', 37, 'open', 'another crazy idea: whole program lint', DEFAULT, 'pros: normal linter like clippy only gives preset rules. And can''t understand what program means. LLM can solve this.

cons: llm may lint at wrong position, and unstable output.

and the information returned by llm may be hard to parse compare to hover function. Because llm need to return the position and modified code.

and the context length also matters.
', DEFAULT, 30024051, '2024-06-27 03:55:33', '2024-07-10 15:36:09', NULL, NULL);
INSERT INTO github.issues VALUES (2356343380, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/29', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/29/comments', 29, 'open', 'Support lsp_client.cancel_requests()', DEFAULT, 'While developing neovim plugin for LSP-AI, I found that despite of asking LSP to cancel previous request, I still received multiple responses from server.', DEFAULT, 2666479, '2024-06-17 04:35:13', '2024-06-17 05:53:55', NULL, NULL);
INSERT INTO github.issues VALUES (2352266185, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/23', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/23/comments', 23, 'closed', 'Added initial post processing to remove duplicate start and end characters', DEFAULT, '…tart and end of the completion response', DEFAULT, 19626586, '2024-06-14 00:37:54', '2024-06-18 20:00:09', '2024-06-14 00:39:29', 19626586);
INSERT INTO github.issues VALUES (2368077996, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/33', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/33/comments', 33, 'closed', 'Issues with special characters, such as `π`', DEFAULT, 'Hi, thanks for this awesome project! I''ve been using it successfully with an ollama instance hosted on a remote machine, and an anthropic api key.

I have many equations in my code, and the LLMs frequently try to use symbols such as π, Γ, or others.
However, in these cases the `lsp-ai` program crashes, with stacktraces such as
```
thread ''tokio-runtime-worker'' panicked at /home/romeo/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lsp-ai-0.3.0/src/transformer_worker.rs:92:60:
byte index 333 is not a char boundary; it is inside ''π'' (bytes 332..334) of `Here''s a Julia function to compute the integral over an ellipsoid:

```julia
# Computes the integral over an ellipsoid
function ellipsoid_integral(a::Float64, b::Float64, c::Float64)
    # a, b, c are the semi-axes of the ellipsoid
    
    # The volume of`[...]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Hope we can fix that, it''s making it rather difficult for me to work, as the same problem happens when existing symbols from my files are parsed by the program.

', DEFAULT, 8644490, '2024-06-22 23:44:33', '2024-08-08 15:39:28', '2024-08-08 15:39:27', 19626586);
INSERT INTO github.issues VALUES (2357003247, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/30', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/30/comments', 30, 'closed', 'Gemini', DEFAULT, 'fix #28 

currently

```
GEMINI_API_KEY=<HERE THE KEY> cargo test --package lsp-ai --bin lsp-ai -- transformer_
backends::gemini::test::gemini_completion_do_generate --exact --show-output
```

works, code needs improve though.
', DEFAULT, 30024051, '2024-06-17 10:36:28', '2024-06-21 15:15:57', '2024-06-21 15:15:56', 19626586);
INSERT INTO github.issues VALUES (2354840741, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/25', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/25/comments', 25, 'closed', 'How do I use this in helix?', DEFAULT, 'Hello,

I have installed lsp-ai

```powershell
❯ (Get-Command lsp-ai).Path
C:\Users\AndreJohansson\.cargo\bin\lsp-ai.exe
```

I have set my openai key

```
❯ $env:OPENAI_API_KEY
sk-proj-...
```

I have used [your example](https://github.com/SilasMarvin/lsp-ai/blob/main/examples/helix/openai-chat.toml) to configure my languages file. I added rust to it.

languages.toml
```toml
...
#################################
## Configuration for languages ##
#################################

[[language]]
name = "python"
language-servers = ["pyright", "lsp-ai"]

[[language]]
name = "rust"
auto-format = true
language-servers = ["rust-analyzer", "lsp-ai"]
```

Then I upen a rust file in helix
```powershell 
hx main.rs
```

How would I then trigger prompts and questions etc?
Please note: I''m a novice helix user, so any keyboard shortcuts and commands are helpful.
', DEFAULT, 2828428, '2024-06-15 12:59:06', '2024-06-15 21:21:05', '2024-06-15 13:53:56', 19626586);
INSERT INTO github.issues VALUES (2345377798, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/18', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/18/comments', 18, 'open', 'Zed Extension', DEFAULT, 'I am trying to use this in [Zed](https://github.com/zed-industries/zed) using the LSP extension. I downloaded the lsp-ai using Cargo, bound it to different file types, and passed the following initialization options:

_(Note: Codegemma is installed on my system using Ollama)_

```json
{
    "memory": {
        "file_store": {}
    },
    "models": {
        "model1": {
            "type": "ollama",
            "model": "codegemma"
        }
    },
    "completion": {
        "model": "model1",
        "parameters": {
            "fim": {
                "start": "<|fim_begin|>",
                "middle": "<|fim_hole|>",
                "end": "<|fim_end|>"
            },
            "max_context": 2000,
            "options": {
                "num_predict": 32
            }
        }
    }
}
```

However, when I tested it, I didn''t get any completions. Here are the LSP logs of lsp-ai:

**Server Logs:**

```
stderr: ERROR lsp_ai::memory_worker: error in memory worker task: Error getting rope slice
stderr: ERROR lsp_ai::transformer_worker: generating response: channel closed
stderr: ERROR lsp_ai::memory_worker: error in memory worker task: Error getting rope slice
stderr: ERROR lsp_ai::transformer_worker: generating response: channel closed
stderr: ERROR lsp_ai::memory_worker: error in memory worker task: Error getting rope slice
stderr: ERROR lsp_ai::transformer_worker: generating response: channel closed
stderr: ERROR lsp_ai::memory_worker: error in memory worker task: Error getting rope slice
stderr: ERROR lsp_ai::transformer_worker: generating response: channel closed
stderr: ERROR lsp_ai::memory_worker: error in memory worker task: Error getting rope slice
stderr: ERROR lsp_ai::transformer_worker: generating response: channel closed
```

**Server Logs (RPC):**

```
// Send:
{"jsonrpc":"2.0","id":8,"method":"textDocument/completion","params":{"textDocument":{"uri":"file:///home/raunak/Documents/zed-lsp-ai/test.py"},"position":{"line":1,"character":5}}}
// Receive:
{"jsonrpc":"2.0","id":8,"error":{"code":-32603,"message":"channel closed"}}
// Send:
{"jsonrpc":"2.0","id":9,"method":"textDocument/completion","params":{"textDocument":{"uri":"file:///home/raunak/Documents/zed-lsp-ai/test.py"},"position":{"line":2,"character":5}}}
// Receive:
{"jsonrpc":"2.0","id":9,"error":{"code":-32603,"message":"channel closed"}}
```

Is this an issue with the editor, lsp-ai, or is it my fault?
', DEFAULT, 71929976, '2024-06-11 04:45:13', '2024-07-26 08:21:18', NULL, NULL);
INSERT INTO github.issues VALUES (2341830542, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/4', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/4/comments', 4, 'closed', 'Using Anthropic API renders some issues', DEFAULT, 'Thanks for this project, I''d love to use this with helix.

Doing some attempts I ran into two issues, the first I was able to track down.

```
2024-06-08T21:55:06.687 helix_lsp::transport [ERROR] lsp-ai <- InternalError: "{\"message\":\"messages.0.tool_calls: Extra inputs are not permitted\",\"type\":\"invalid_request_error\"}"
```

I made a fork and removed both references to tool_calls in config.rs; this fixed that issue.
I''m no Rustacean, so not sure how to implement that nicely, but I hope it points you in some useful direction.

---

The second issue I''m not sure how to tackle…
I get results from the Anthropic-API, but they are cut off at (I believe…) about 155 tokens.

For example, if I enter

```ts
const monthNames = <CURSOR>
```

…it returns a (cut-off) list of monthNames.

The following, however, works fine.

```ts
const monthNamesAbbreviated = <CURSOR>
```', DEFAULT, 68561, '2024-06-08 20:46:42', '2024-06-12 01:14:17', '2024-06-11 16:03:42', 19626586);
INSERT INTO github.issues VALUES (2346449494, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/20', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/20/comments', 20, 'closed', 'Update cargo lockfile for frozen builds', DEFAULT, 'Running `cargo build --frozen` fails for release 0.2.0. I am trying to package 0.2.0 for nixos, but the lockfile being behind makes it so that I cannot do so. An easy fix is to update the lockfile and bump the version to (say) 0.2.1.

Steps to reproduce:

Run `cargo build --frozen`

Output:
```
error: the lock file /path/to/lsp-ai/Cargo.lock needs to be updated but --frozen was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --frozen flag and use --offline instead.
```', DEFAULT, 38045210, '2024-06-11 13:30:39', '2024-06-11 16:07:41', '2024-06-11 15:53:13', 19626586);
INSERT INTO github.issues VALUES (2343972535, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/17', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/17/comments', 17, 'open', 'doc: document using nvim-lspconfig', DEFAULT, 'This PR edits the example configuration for neovim to use the nvim-lspconfig support.

Closes #2 ', DEFAULT, 304428, '2024-06-10 13:45:54', '2024-10-12 13:50:39', NULL, NULL);
INSERT INTO github.issues VALUES (2342506239, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/13', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/13/comments', 13, 'closed', 'How to start model?', DEFAULT, 'I''ve installed LSP-AI on my local

```sh
$ which lsp-ai

/Users/future/.cargo/bin/lsp-ai
```

But I don''t know how to start it.
when I run 
```sh
$ /Users/future/.cargo/bin/lsp-ai             
```

The console hangs. If I press a key it exits immediately. 
Not sure what to do next. Please advise.

Thanks for your work!', DEFAULT, 16382165, '2024-06-09 21:48:25', '2024-06-10 03:30:23', '2024-06-09 23:02:33', 19626586);
INSERT INTO github.issues VALUES (2342117668, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/8', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/8/comments', 8, 'closed', 'Allow the llama_cpp config to specify model file path', DEFAULT, NULL, DEFAULT, 19626586, '2024-06-09 09:10:40', '2024-06-09 15:07:01', '2024-06-09 15:07:01', 19626586);
INSERT INTO github.issues VALUES (2341796211, 723094714, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/3', 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/3/comments', 3, 'closed', 'RAG for code', DEFAULT, 'Really interesting project – I think an AI-focused LSP makes a lot of sense as a standalone service.

> Implement semantic search-powered context building (This could be incredibly cool and powerful). Planning to use [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) to chunk code correctly.

You might want to check out https://github.com/getgrit/gritql by @morgante et al. It''s a Rust-based wrapper around tree-sitter that I used for my last project doing RAG for code and was really impressed with it.

Cheers 👋 ', DEFAULT, 552829, '2024-06-08 19:18:19', '2024-08-08 14:08:10', '2024-08-08 14:08:09', 19626586);


--
-- Data for Name: comments; Type: TABLE DATA; Schema: github; Owner: silas
--

INSERT INTO github.comments VALUES (2363963416, 2511293260, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2363963416', 'This seems like an issue with your permissions with cargo and rust. I would remove your .cargo directory and reinstall rust. That should fix it.', DEFAULT, '2024-09-20 15:13:31', '2024-09-20 15:13:31');
INSERT INTO github.comments VALUES (2303353982, 2458890504, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2303353982', 'I don''t see any errors in the logs. Can you type in your editor? Use it until you would expect to see a completion and share those logs.', DEFAULT, '2024-08-22 00:39:54', '2024-08-22 00:39:54');
INSERT INTO github.comments VALUES (2363961358, 2535072112, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2363961358', 'This could be pretty cool and we actually already have most of the hard part written: https://github.com/SilasMarvin/lsp-ai/blob/main/crates/lsp-ai/src/memory_backends/vector_store.rs

We would need to write some configuration for which directories to crawl and maybe some way to watch for changes to them?

Feel free to take a swing at it!', DEFAULT, '2024-09-20 15:12:27', '2024-09-20 15:12:27');
INSERT INTO github.comments VALUES (2367857818, 2542198034, 40359643, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2367857818', '#68 ', DEFAULT, '2024-09-23 10:48:28', '2024-09-23 10:48:28');
INSERT INTO github.comments VALUES (2373065968, 2546591573, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2373065968', 'This is an issue with how neovim converts objects into JSON. By setting `file_store` to `{}` you are setting it to an empty list in lua when it should actually be an empty object. This causes deserialization issues on the server. Here is an example neovim config I have used and confirmed it works:

```lua
-- Set leader
vim.g.mapleader = " "
vim.g.maplocalleader = "\\"

-- The init_options
local lsp_ai_init_options_json = [[
{
  "memory": {
    "file_store": {}
  },
  "models": {
    "model1": {
      "type": "anthropic",
      "chat_endpoint": "https://api.anthropic.com/v1/messages",
      "model": "claude-3-5-sonnet-20240620",
      "auth_token_env_var_name": "ANTHROPIC_API_KEY"
    }
  },
  "actions": [
    {
      "trigger": "!C",
      "action_display_name": "Chat",
      "model": "model1",
      "parameters": {
        "max_context": 4096,
        "max_tokens": 4096,
        "system": "You are an AI coding assistant. Your task is to complete code snippets. The user''s cursor position is marked by \"<CURSOR>\". Follow these steps:\n\n1. Analyze the code context and the cursor position.\n2. Provide your chain of thought reasoning, wrapped in <reasoning> tags. Include thoughts about the cursor position, what needs to be completed, and any necessary formatting.\n3. Determine the appropriate code to complete the current thought, including finishing partial words or lines.\n4. Replace \"<CURSOR>\" with the necessary code, ensuring proper formatting and line breaks.\n5. Wrap your code solution in <answer> tags.\n\nYour response should always include both the reasoning and the answer. Pay special attention to completing partial words or lines before adding new lines of code.\n\n<examples>\n<example>\nUser input:\n--main.py--\n# A function that reads in user inpu<CURSOR>\n\nResponse:\n<reasoning>\n1. The cursor is positioned after \"inpu\" in a comment describing a function that reads user input.\n2. We need to complete the word \"input\" in the comment first.\n3. After completing the comment, we should add a new line before defining the function.\n4. The function should use Python''s built-in `input()` function to read user input.\n5. We''ll name the function descriptively and include a return statement.\n</reasoning>\n\n<answer>t\ndef read_user_input():\n    user_input = input(\"Enter your input: \")\n    return user_input\n</answer>\n</example>\n\n<example>\nUser input:\n--main.py--\ndef fibonacci(n):\n    if n <= 1:\n        return n\n    else:\n        re<CURSOR>\n\n\nResponse:\n<reasoning>\n1. The cursor is positioned after \"re\" in the ''else'' clause of a recursive Fibonacci function.\n2. We need to complete the return statement for the recursive case.\n3. The \"re\" already present likely stands for \"return\", so we''ll continue from there.\n4. The Fibonacci sequence is the sum of the two preceding numbers.\n5. We should return the sum of fibonacci(n-1) and fibonacci(n-2).\n</reasoning>\n\n<answer>turn fibonacci(n-1) + fibonacci(n-2)</answer>\n</example>\n</examples>\n",
        "messages": [
          {
            "role": "user",
            "content": "{CODE}"
          }
        ]
      },
      "post_process": {
        "extractor": "(?s)<answer>(.*?)</answer>"
      }
    }
  ]
}
]]

-- The configuration
local lsp_ai_config = {
  cmd = { ''lsp-ai'', ''--use-seperate-log-file'' },
  root_dir = vim.loop.cwd(),
  init_options = vim.fn.json_decode(lsp_ai_init_options_json),
}

-- Start lsp-ai when opening a buffer
vim.api.nvim_create_autocmd("BufEnter", {
  callback = function(args)
    local bufnr = args.buf
    local client = vim.lsp.get_active_clients({bufnr = bufnr, name = "lsp-ai"})
    if #client == 0 then
      vim.lsp.start(lsp_ai_config, {bufnr = bufnr})
    end
  end,
})

-- Key mapping for code actions
vim.api.nvim_set_keymap(''n'', ''<leader>c'', ''<cmd>lua vim.lsp.buf.code_action()<CR>'', {noremap = true, silent = true})
```', DEFAULT, '2024-09-25 05:37:22', '2024-09-25 05:37:22');
INSERT INTO github.comments VALUES (2346473845, 2521844715, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2346473845', 'I''m not very familiar with the Kate editor. If the Kate editor supports the language sever protocol it will work in Kate. You will need to look at the Kate docs for setting up a custom language server and add LSP-AI. You will need to find out how to pass initializationOptions and pass the config you want. An example of that is outside the scope of an issue with LSP-AI so I am going to close this, but feel free to make a pr to add an example with Kate.', DEFAULT, '2024-09-12 14:32:44', '2024-09-12 14:32:44');
INSERT INTO github.comments VALUES (2351901243, 2521844715, 13515498, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2351901243', '@SilasMarvin Thanks for your reply, I used the binary version and added the setting to the initializationOptions but got the error. Is there something I did wrong?

`ERROR lsp_ai: Unsupported command - see the wiki for a list of supported commands: Request { id: RequestId(I32(8)), method: "textDocument/documentSymbol", params: Object {"textDocument": Object {"uri": String("file:///E:/test.md")}} }`

`{
  "servers": {
    "markdown": {
      "command": ["D:/Program Files/lsp_ai/lsp-ai.exe", "--stdio"],
      "initializationOptions": {
  "memory": {
    "file_store": {}
  },
  "models": {
    "model1": {
      "type": "ollama",
      "model": "phi3.5"
    }
  },
  "chat": [
    {
      "trigger": "!C",
      "action_display_name": "Chat",
      "model": "model1",
      "parameters": {
        "max_context": 4096,
        "max_tokens": 1024,
        "messages": [
          {
            "role": "system",
            "content": "You are a code assistant chatbot. The user will ask you for assistance coding and you will do you best to answer succinctly and accurately"
          }
        ]
      }
    }
  ]
}
    }
  }
}
`', DEFAULT, '2024-09-16 01:45:21', '2024-09-16 01:45:21');
INSERT INTO github.comments VALUES (2481746002, 2561734813, 837049, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2481746002', 'I can get completion to work with llama.cpp by using the `open_ai` backend.

Firstly, for completion, you can use a Qwen2.5 Coder base model, instead of an instruct model. The models from https://huggingface.co/mradermacher/Qwen2.5-32B-GGUF should be alright. 

With a Qwen2.5 Coder base model, the FIM tokens should be `<|fim_prefix|>`, `<|fim_suffix|>`, and `<|fim_middle|>`.

Then, due to a compatibility issue with llama.cpp /v1/completions endpoint (see https://github.com/ggerganov/llama.cpp/discussions/9219), we will need a small patch for lsp-ai:
```diff
diff --git a/crates/lsp-ai/src/transformer_backends/open_ai/mod.rs b/crates/lsp-ai/src/transformer_backends/open_ai/mod.rs
index c75b580..61b5298 100644
--- a/crates/lsp-ai/src/transformer_backends/open_ai/mod.rs
+++ b/crates/lsp-ai/src/transformer_backends/open_ai/mod.rs
@@ -57,11 +57,6 @@ pub(crate) struct OpenAI {
     configuration: config::OpenAI,
 }
 
-#[derive(Deserialize, Serialize)]
-pub(crate) struct OpenAICompletionsChoice {
-    text: String,
-}
-
 #[derive(Deserialize, Serialize)]
 pub(crate) struct OpenAIError {
     pub(crate) error: Value,
@@ -69,7 +64,7 @@ pub(crate) struct OpenAIError {
 
 #[derive(Deserialize, Serialize)]
 pub(crate) struct OpenAIValidCompletionsResponse {
-    pub(crate) choices: Vec<OpenAICompletionsChoice>,
+    pub(crate) content: String,
 }
 
 #[derive(Deserialize, Serialize)]
@@ -163,7 +158,7 @@ impl OpenAI {
         );
         match res {
             OpenAICompletionsResponse::Success(mut resp) => {
-                Ok(std::mem::take(&mut resp.choices[0].text))
+                Ok(std::mem::take(&mut resp.content))
             }
             OpenAICompletionsResponse::Error(error) => {
                 anyhow::bail!(

```

I use micro instead of helix, and this is the relevant settings:
```json
    "lsp.python": "{\"memory\":{\"file_store\":{}},\"models\":{\"model1\":{\"type\":\"open_ai\",\"completions_endpoint\":\"http://1.2.3.4:8080/v1/completions\",\"model\":\"Qwen/Qwen2.5-Coder-32B\",\"auth_token\":\"\"}},\"completion\":{\"model\":\"model1\",\"parameters\":{\"max_context\":4096,\"max_tokens\":512,\"top_p\":0.01,\"fim\":{\"start\":\"\u003c|fim_prefix|\u003e\",\"middle\":\"\u003c|fim_suffix|\u003e\",\"end\":\"\u003c|fim_middle|\u003e\"}}}}",
    "lsp.server": "python=lsp-ai --use-seperate-log-file,go=gopls,typescript=deno lsp,rust=rls",
```', DEFAULT, '2024-11-18 01:26:36', '2024-11-18 01:26:36');
INSERT INTO github.comments VALUES (2414340970, 2588761718, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2414340970', 'Thanks for checking out my project! The example you are referencing uses Anthropic not OpenAI. Anthropic requires system as a separate key', DEFAULT, '2024-10-15 15:33:29', '2024-10-15 15:33:29');
INSERT INTO github.comments VALUES (2303553394, 2479627865, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2303553394', 'I actually ran into this error the other day. This is caused by how lua converts tables into JSON. Lua converts the `{}` into a JSON `[]` which is incorrect as the LSP-AI server expects `file_store` to be an object.

Here is an example of some init options that will work copied directly from my init.lua file:
```lua
-- Set leader
vim.g.mapleader = " "
vim.g.maplocalleader = "\\"

-- The init_options
local lsp_ai_init_options_json = [[
{
  "memory": {
    "file_store": {}
  },
  "models": {
    "model1": {
      "type": "anthropic",
      "chat_endpoint": "https://api.anthropic.com/v1/messages",
      "model": "claude-3-5-sonnet-20240620",
      "auth_token_env_var_name": "ANTHROPIC_API_KEY"
    }
  },
  "actions": [
    {
      "trigger": "!C",
      "action_display_name": "Chat",
      "model": "model1",
      "parameters": {
        "max_context": 4096,
        "max_tokens": 4096,
        "system": "You are an AI coding assistant. Your task is to complete code snippets. The user''s cursor position is marked by \"<CURSOR>\". Follow these steps:\n\n1. Analyze the code context and the cursor position.\n2. Provide your chain of thought reasoning, wrapped in <reasoning> tags. Include thoughts about the cursor position, what needs to be completed, and any necessary formatting.\n3. Determine the appropriate code to complete the current thought, including finishing partial words or lines.\n4. Replace \"<CURSOR>\" with the necessary code, ensuring proper formatting and line breaks.\n5. Wrap your code solution in <answer> tags.\n\nYour response should always include both the reasoning and the answer. Pay special attention to completing partial words or lines before adding new lines of code.\n\n<examples>\n<example>\nUser input:\n--main.py--\n# A function that reads in user inpu<CURSOR>\n\nResponse:\n<reasoning>\n1. The cursor is positioned after \"inpu\" in a comment describing a function that reads user input.\n2. We need to complete the word \"input\" in the comment first.\n3. After completing the comment, we should add a new line before defining the function.\n4. The function should use Python''s built-in `input()` function to read user input.\n5. We''ll name the function descriptively and include a return statement.\n</reasoning>\n\n<answer>t\ndef read_user_input():\n    user_input = input(\"Enter your input: \")\n    return user_input\n</answer>\n</example>\n\n<example>\nUser input:\n--main.py--\ndef fibonacci(n):\n    if n <= 1:\n        return n\n    else:\n        re<CURSOR>\n\n\nResponse:\n<reasoning>\n1. The cursor is positioned after \"re\" in the ''else'' clause of a recursive Fibonacci function.\n2. We need to complete the return statement for the recursive case.\n3. The \"re\" already present likely stands for \"return\", so we''ll continue from there.\n4. The Fibonacci sequence is the sum of the two preceding numbers.\n5. We should return the sum of fibonacci(n-1) and fibonacci(n-2).\n</reasoning>\n\n<answer>turn fibonacci(n-1) + fibonacci(n-2)</answer>\n</example>\n</examples>\n",
        "messages": [
          {
            "role": "user",
            "content": "{CODE}"
          }
        ]
      },
      "post_process": {
        "extractor": "(?s)<answer>(.*?)</answer>"
      }
    }
  ]
}
]]

-- The configuration
local lsp_ai_config = {
  cmd = { ''lsp-ai'' },
  root_dir = vim.loop.cwd(),
  init_options = vim.fn.json_decode(lsp_ai_init_options_json),
}

-- Start lsp-ai when opening a buffer
vim.api.nvim_create_autocmd("BufEnter", {
  callback = function(args)
    local bufnr = args.buf
    local client = vim.lsp.get_active_clients({bufnr = bufnr, name = "lsp-ai"})
    if #client == 0 then
      vim.lsp.start(lsp_ai_config, {bufnr = bufnr})
    end
  end,
})

-- Key mapping for code actions
vim.api.nvim_set_keymap(''n'', ''<leader>c'', ''<cmd>lua vim.lsp.buf.code_action()<CR>'', {noremap = true, silent = true})
```

Basically use the `vim.fn.json_decode` function and it will fix it.

Let me know if you have any other issues or suggestions. Thanks for checking it out!', DEFAULT, '2024-08-22 02:24:38', '2024-08-22 02:25:46');
INSERT INTO github.comments VALUES (2302404260, 2478355341, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2302404260', 'Thanks for bringing this up! I recently added some command line arguments to LSP-AI for file logging and apparently vscode was always passing the `--stdio` argument which isn''t a real argument we accept as we always communicate over stdio. I added `--stdio` as an argument though it doesn''t actually do anything for now. 

Patched in: https://github.com/SilasMarvin/lsp-ai/releases/tag/v0.6.1', DEFAULT, '2024-08-21 15:41:31', '2024-08-21 15:41:31');
INSERT INTO github.comments VALUES (2501935374, 2575475531, 21687187, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2501935374', 'As much as I remember, Copilot is based on OpenAIs ChatGPT. I remember [`helix-gpt`](https://github.com/leona/helix-gpt) having a [GitHub Copilot](https://github.com/leona/helix-gpt/blob/master/src/providers/github.ts) integration. I looked a bit more at the code and found out, that they are using the https://api.githubcopilot.com/chat/completions endpoint. I guess you can just use the `open_ai` examples and replace the endpoint with the one from githubcopilot. [Here](https://bootk.id/posts/copilot/) is some more information about the API:

> After doing more analysis, I determined that the API endpoint at `https://api.githubcopilot.com/chat/completions` was nothing more than an OpenAI API proxy, with a different auth wrapper (which authenticated using the Copilot bearer token instead of an OpenAI key). I could query any model, with nearly identical behavior to the real OpenAI API.

I tried it out with helix, but I am getting the following error:
```log
2024-11-26T21:55:58.707 helix_lsp::transport [ERROR] lsp-ai err <- "ERROR lsp_ai::transformer_worker: generating response: error decoding response body: expected value at line 1 column 1\n"
2024-11-26T21:55:58.707 helix_lsp::transport [ERROR] lsp-ai err <- "\n"
2024-11-26T21:55:58.707 helix_lsp::transport [ERROR] lsp-ai err <- "Caused by:\n"
2024-11-26T21:55:58.707 helix_lsp::transport [ERROR] lsp-ai err <- "    expected value at line 1 column 1\n"
2024-11-26T21:55:58.707 helix_lsp::transport [ERROR] lsp-ai <- InternalError: error decoding response body: expected value at line 1 column 1

```

**EDIT** My current lsp-ai config for helix can be found in [flake repo](https://github.com/arunoruto/flake/blob/main/modules/home-manager/server/cli/helix/languages/lsp-ai.nix). You can kinda read like json, just replace the `=` with `:` and remove the `;` at the end;

**EDIT 2** I looked a bit more into the whole Open AI API compatibility. While api.githubcopilot.com is being used now, it used to be a different endpoint until the beginning of 2024. According to [GitHubs changelog](https://github.blog/changelog/2024-01-18-copilot-january-18th-update/), they used to have copilot-proxy.githubusercontent.com as their API endpoint. This is also referenced in [this line](https://github.com/leona/helix-gpt/blob/2a047347968e63ca55e2bf0db74e80130941f1f8/src/providers/github.ts#L168) of helix-gpt.
What does this mean? Chat should work with the https://api.githubcopilot.com/chat/completions endpoint, but completion will not. Doing some manual curl commands, the /chat/completions endpoint expects a `messages` entry in the data, while for completions a `prompt` is used. I tried manually submitting a request to https://copilot-proxy.githubusercontent.com/v1/engines/copilot-codex/completions as described by helix-gpt, but my token isn''t the right one for such requests... Looking at the code, the copilot API token is used to obtain a session token, which is then used further to make requests.
tldr;
- Copilot chat should work using https://api.githubcopilot.com/chat/completions as an endpoint and the output of `gh auth token` as the token value. I used `set -gx GH_AUTH_TOKEN $(gh auth token)` to set the env variable in fish.
- Completions need a different endpoint which requires the `prompt` property to be set.', DEFAULT, '2024-11-26 21:06:29', '2024-11-27 00:09:52');
INSERT INTO github.comments VALUES (2315413123, 2491879042, 8382834, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2315413123', 'Thanks for the explanation, that makes sense! :)', DEFAULT, '2024-08-28 14:03:28', '2024-08-28 14:03:28');
INSERT INTO github.comments VALUES (2401931751, 2575475531, 13739892, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2401931751', 'It is important to note that **GitHub Copilot Chat** supports various models, including `GPT-4`, `GPT-4o`, `o1`, and `o1-mini`. For more detailed information and updates, you can refer to the [GitHub Copilot Chat changelog](https://github.blog/changelog/label/copilot-chat/).

Given the widespread use and active subscriptions to **GitHub Copilot Chat** among developers, it would be highly beneficial for the `lsp-ai` plugin to incorporate support for these models. This integration would enhance the development experience by leveraging the advanced capabilities of **GitHub Copilot Chat**.
', DEFAULT, '2024-10-09 10:25:59', '2024-10-09 10:25:59');
INSERT INTO github.comments VALUES (2271988896, 2426171444, 90320947, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2271988896', '> the problem is copilot does not uses the LS protocol, it uses a proprietary protocol to communicate with the model.

Got it, but the project I linked uses it as LSP, maybe it can be used as reference, either way I stopped using copilot in favour of Claude :P', DEFAULT, '2024-08-06 19:24:36', '2024-08-06 19:24:36');
INSERT INTO github.comments VALUES (2169691622, 2354840741, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2169691622', '> Hello,
> 
> I have installed lsp-ai
> 
> ```powershell
> ❯ (Get-Command lsp-ai).Path
> C:\Users\AndreJohansson\.cargo\bin\lsp-ai.exe
> ```
> 
> I have set my openai key
> 
> ```
> ❯ $env:OPENAI_API_KEY
> sk-proj-...
> ```
> 
> I have used [your example](https://github.com/SilasMarvin/lsp-ai/blob/main/examples/helix/openai-chat.toml) to configure my languages file. I added rust to it.
> 
> languages.toml
> 
> ```toml
> ...
> #################################
> ## Configuration for languages ##
> #################################
> 
> [[language]]
> name = "python"
> language-servers = ["pyright", "lsp-ai"]
> 
> [[language]]
> name = "rust"
> auto-format = true
> language-servers = ["rust-analyzer", "lsp-ai"]
> ```
> 
> Then I upen a rust file in helix
> 
> ```powershell
> hx main.rs
> ```
> 
> How would I then trigger prompts and questions etc? Please note: I''m a novice helix user, so any keyboard shortcuts and commands are helpful.

Completions are triggered when typing in Helix. They pop up in the same window that completions from the rust-analyzer show up in. There are some configuration options available. See the `completion-*` keys: https://docs.helix-editor.com/configuration.html

For a good test to make sure your completions are working you should be able to type something like:
```rust
fn multiply_two_numbers
```
If you pause for a second with your cursor directly after the `s` in Insert mode, you should get a completion that pops up.

When helix does get plugins support we are planning to write our own Helix plugin that will offer inline completion like our VS Code plugin.

If LSP-AI completions are not displaying for you, please follow the Debug section of the wiki and share the output of the helix log file.

Thanks for checking LSP-AI out!', DEFAULT, '2024-06-15 13:53:56', '2024-06-15 13:55:18');
INSERT INTO github.comments VALUES (2272625763, 2451684749, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2272625763', 'I''ll look into the compilation issues that is strange its not working on Ubuntu 22.04. 

The home section of the wiki has a nice getting started section: https://github.com/SilasMarvin/lsp-ai/wiki

If you want to use it with VS Code you should install our official plugin: https://github.com/SilasMarvin/lsp-ai/wiki/Plugins', DEFAULT, '2024-08-07 05:06:21', '2024-08-07 05:06:21');
INSERT INTO github.comments VALUES (2243670174, 2421318028, 15968876, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2243670174', 'Happy to hear you agree!

> [... ] we may want to provide some kind of post process regex response matcher

Funnily enough, I actually typed up another issue right after arguing for exactly this, but I started second-guessing its true necessity halfway through and left it at that.

> If we send it markdown will it respond with markdown? 

Not necessarily. In fact, I''ve empirically verified as much by modifying the user messages in the example chat mode prompt in the wiki so that they all contain Markdown code blocks, while leaving the assistant messages untouched. The result: no difference in the generated response than before, i.e. no triple backticks to parse out. So it seems a few in-context example pairs where the answer is ''bare'' sufficiently conditions the final response to also be bare.

On the other hand, few-shot learning leads to slower responses due to the increased prompt length. I just did an experiment with Claude 3.5 Sonnet (in the browser dashboard):

<img width="1426" alt="Screenshot 2024-07-22 at 21 46 03" src="https://github.com/user-attachments/assets/04ec8e70-237c-45a1-9670-9124f451d602">

So by starting the chat completion with a final prefilled assistant message, you get a markdown code block. The generated code indentation is correct when the output is formatted as a code block. Interestingly, when I repeat the example without the prefilled assistant message, I get a bare code response with _unindented_ code. Not what you''d want here.

> This extractor would actually be really useful for chain of though prompting:

Yes! I believe that''s where the real value of an additional post-processing mechanism lies. It occurred to me that the system message in the examples asks for a step-by-step response, but none of the few-shot examples demonstrate it, nor is there any way to separate the thought part of the response from the answer afterward.

Changing this would allow for an enormous amount of creativity.

> preset list of extractors like Markdown


... Markdown code blocks, the Claude XML tags, ... My thoughts quickly wonder to JSON, but that would call for something different than regex. So maybe let''s restrict the scope to regex and provide presets, or just well-documented examples, of those two.

> If you want to take a crack at adding it

With all the Rust-based projects I''m making feature requests to lately, it''s starting to seem more and more likely that I''m going to have to start learning Rust sooner or later. But right _now_ I have zero knowledge of it, so I''m afraid I won''t be much help here. But I''m definitely down to brainstorm on the overall design and help write documentation', DEFAULT, '2024-07-22 19:31:27', '2024-07-22 19:49:32');
INSERT INTO github.comments VALUES (2370051425, 2537650065, 180627245, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2370051425', '> > This is a really awesome pr! I''ve used nix a little bit but not enough to know the proper ways to do things. Ss there some kind of way to make sure this builds correctly in our CI?
> 
> I am not sure I understand: Is there a desire to use nix instead for CI? As these changes aren''t _necessary_ or intended to replace a working CI, unless it''s needed.

I think he''s referring to ensuring that the nix package builds (which theoretically could fail even if the rust package can compile via the normal imperative commands). There seems to be [some documentation](https://docs.cachix.org/continuous-integration-setup/) already over using nix in CI, ~~and if there are no immediate plans to merge into the `nixpkgs` repo~~ (updated based on latest comment), there''s an [official tutorial](https://nix.dev/guides/recipes/continuous-integration-github-actions) on using GitHub Actions to automatically push binaries to a Cachix binary cache (to prevent everyone building the same derivation on their own systems) if this project were to adopt one.', DEFAULT, '2024-09-24 03:39:36', '2024-09-24 03:40:50');
INSERT INTO github.comments VALUES (2183810585, 2367561345, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2183810585', 'Hey Cameron thanks for checking it out! I''ve never added anything to home-manager but we absolutely can. Its not super high on the priority list for me right now, but if you want to make a pr adding a flake.nix file or whatever is required I would be happy to merge it!

I''m not sure why it isn''t running in your editor. Make sure to add it to your languages.toml file. You can see some example in the examples directory of the repo.', DEFAULT, '2024-06-22 05:20:12', '2024-06-22 05:20:12');
INSERT INTO github.comments VALUES (2192793368, 2373729518, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2192793368', '> some other ideas:
> 
> hover action add test case for functions.
> 
> hover action refactor functions.

Code actions could be incredibly cool! I really love this. Its not the top priority on my list, but I will start looking into it. We should come up with a list of codeactions we want to provide, or we should come up with a way for users to be able to configure their own?', DEFAULT, '2024-06-26 23:40:04', '2024-06-26 23:40:04');
INSERT INTO github.comments VALUES (2364999972, 2537650065, 6314611, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2364999972', '> This is a really awesome pr! I''ve used nix a little bit but not enough to know the proper ways to do things. Ss there some kind of way to make sure this builds correctly in our CI?

I am not sure I understand: Is there a desire to use nix instead for CI? As these changes aren''t _necessary_ or intended to replace a working CI, unless it''s needed.', DEFAULT, '2024-09-21 04:51:15', '2024-09-21 04:51:15');
INSERT INTO github.comments VALUES (2294956925, 2442606878, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2294956925', 'This is actually a bad idea as that assumes they are running locally. I think this is something they should handle themselves', DEFAULT, '2024-08-17 19:42:10', '2024-08-17 19:42:10');
INSERT INTO github.comments VALUES (2173983541, 2357003247, 30024051, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2173983541', 'now

```
---- transformer_backends::gemini::test::gemini_completion_do_generate stdout ----
[src/transformer_backends/gemini.rs:203:9] response.generated_text = "\"```python\\ndef test_context():\\n    \\\"\\\"\\\"\\n    This function serves as a template for writing unit tests using the ''context'' pattern.\\n\\n    The ''context'' pattern helps organize test cases by grouping them based on the state they set up.\\n\\n    To use this template:\\n\\n    1. Replace the \\\"pass\\\" statement with your test code.\\n    2. Define the specific context you want to test within a \\\"with\\\" block.\\n    3. Use the `self.assertEqual()`, `self.assertTrue()`, etc. methods to assert the expected outcomes.\\n\\n    Example:\\n\\n    ```python\\n    def test_context(self):\\n        with self.subTest(\\\"Positive numbers\\\"):\\n            self.assertEqual(add(2, 3), 5)\\n\\n        with self.subTest(\\\"Negative numbers\\\"):\\n            self.assertEqual(add(-2, -3), -5)\\n    ```\\n    \\\"\\\"\\\"\\n    pass\\n```\\n\\n**Explanation:**\\n\\n* **`def test_context():`**: This defines a function named `test_context` which will contain your test cases.\\n* **Docstring**:  The docstring explains the purpose and usage of the function.\\n* **`with self.subTest(\\\"Positive numbers\\\"):`**: This block creates a subtest labeled \\\"Positive numbers\\\". It''s useful for grouping related assertions and providing informative error messages.\\n* **`self.assertEqual(add(2, 3), 5)`**: This assertion checks if the result of calling `add(2, 3)` is equal to 5.\\n* **`with self.subTest(\\\"Negative numbers\\\"):`**: Another subtest block for testing negative numbers.\\n* **`self.assertEqual(add(-2, -3), -5)`**:  This assertion checks if the result of calling `add(-2, -3)` is equal to -5.\\n\\n**Key Points:**\\n\\n* **Context-Driven Testing:** The `with self.subTest(...)` pattern allows you to group tests based on specific contexts, making your tests more organized and readable.\\n* **Subtests:** Subtests provide informative error messages that specify the context in which the failure occurred.\\n* **Assertions:**  Use assertion methods (like `self.assertEqual()`, `self.assertTrue()`, etc.) to verify the expected outcomes of your tests.\\n\\n**Remember to replace the `pass` statement with your actual test code!** \\n\""
```

and

```
---- transformer_backends::gemini::test::gemini_chat_do_generate stdout ----
[src/transformer_backends/gemini.rs:235:9] &response = "\"Well, I mean, obviously I love winter!  It''s the only time of year I get to exist!  But I hear spring is pretty nice, too, though I wouldn''t know.  I tend to melt a little that time of year.  *shivers*  Just thinking about it makes me want to snuggle up next to my trusty snow-ball companion! \\n\""
```
', DEFAULT, '2024-06-17 17:46:11', '2024-06-17 17:46:11');
INSERT INTO github.comments VALUES (2254232419, 2426171444, 90320947, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2254232419', '> There are currently four different types of configurable models:
> 
> - llama.cpp models
> - Ollama models
> - OpenAI API compatible models
> - Anthropic API compatible models
> - Mistral AI API compatible models

I''m suggesting using GIthub Copilot as it has a predictable price, I was pointing to that project that successfully used Github Copilot as LSP.', DEFAULT, '2024-07-27 19:18:49', '2024-07-27 19:18:49');
INSERT INTO github.comments VALUES (2282222094, 2458890504, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2282222094', 'Thank you for your thoughts. This is not a problem I will be able to 100% solve myself. I appreciate that you value the project enough to share your ideas, and I agree with you on most points. 

The current configuration guide is awful. For users to be successful they must be aware of and understand how language servers work, how to add them to their editor, and how to debug them. This is an unrealistic expectation.

You are right a good guide going from zero to in editor chatting / completion would be incredibly helpful. I will get this out. 

I don’t think we can emulate helix’s batteries included methodology here. There are too many moving pieces, and to a certain extent, I don’t think it’s bad it requires some thought to configure as this a powerful useful tool for developers if used correctly. 

What I want is to create a online configurator that asks a series of questions like:
1. Do you want to enable in editor chatting (If so we ask what model, etc..)
2. What editor are you using
3. Do you want to enable inline-completion (if so we ask what model, etc…)

What do you think of an online configurator that not only outputs (in the case of helix) the languages.toml but explains what it does, how to use it, etc….? We can extend this to work for Neovim, and other popular text editors. I want to remove the barrier of having to read through the configuration section of the wiki unless they want to get into the weeds.

The logging is awful right now. I have a pr coming out this weekend (I think) that creates our own logging file which also includes all completion requests sent to llms in a nice user readable format. It’s a massive improvement. I’m still debating whether to make it the default log file, or if it should be a command line parameter that enables it. It will exist in the users cache directory (~/.cache on linux).

Once again thank you for your thoughts! It’s conversations like this that will make this project great.', DEFAULT, '2024-08-10 17:39:21', '2024-08-10 17:39:21');
INSERT INTO github.comments VALUES (2479158880, 2537650065, 6033387, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2479158880', 'Chiming here to show interest.

Any updates on this? @ProjectInitiative @mergemoveagree ', DEFAULT, '2024-11-15 15:25:56', '2024-11-15 15:25:56');
INSERT INTO github.comments VALUES (2532608114, 2458890504, 7916909, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2532608114', 'As soon as I saw "a school header everywhere in my projects" and `C` as your lang, I was like, is this a wild 42 Student?
`c_formatter_42` confirmed that.
I ran into the same problem using codelama on my old 42 project code, so you''re not alone.
Something about this model really does not like 42 header, even when it does complete, the results seem very degraded in files that have it.', DEFAULT, '2024-12-10 18:49:31', '2024-12-10 18:50:22');
INSERT INTO github.comments VALUES (2192792181, 2373729518, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2192792181', '> I like the idea!
> 
> > cons: increase the cost, the ai will generate different answers each time, the respond speed, etc.
> 
> We could cache the responses to make it fast on second hover & keep the result consistent. We could provide a "clear cache" code action in the lsp to clear the generated text if it is unsatisfactory, either global or based on the cursor position to clear the llm-generated content. Could be regenerate rather than clear cache as well (the code action).
> 
> @SilasMarvin this would play well with what is stored in the memory backend & could add context to the function by fetching from the vector store.

Providing documentation is a really cool idea. I don''t know how this plays with other language servers. If they have a language server that already provides documentation will the editor pull from both? If I understand correctly we would need to support the `textDocument/hover` capability unless we want to provide this as a custom function?', DEFAULT, '2024-06-26 23:38:28', '2024-06-26 23:38:28');
INSERT INTO github.comments VALUES (2370040790, 2537650065, 180627245, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2370040790', 'Hey! Love to see work on packaging `lsp-ai` for nixpkgs! I currently am using my own local package (https://github.com/mergemoveagree/nixos-configuration/tree/d9baa33e94b2680957defcc776704e4443266417/pkgs/lsp-ai) that was initialized with `nix-init`. While I am also relatively new to nix, I''ve combed through a mountain of different package flakes to try and find idiomatic styling.

Here are some differences between our flakes that I think would be beneficial to add:
- use `outputHashes` rather than `allowBuiltinFetchGit` -- the hashes are in my "callBuilder.nix" file so you could just copy them from there
- include a "default.nix" generated by `flake-compat` for the folks who don''t want to switch to `nixos-unstable` just yet
- Consider moving the package into an overlay -- I see this in a lot of other flakes for big projects

I''m not the most knowledgeable, but I''m happy to try and answer any questions or help research things we both know!', DEFAULT, '2024-09-24 03:31:59', '2024-09-24 03:31:59');
INSERT INTO github.comments VALUES (2192069712, 2374752627, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2192069712', 'Wow this is really incredible thank you for adding all this! I am not very familiar with GitHub actions and have learned a lot reading through this.

I''m not 100% sure why we changed `pub` to `pub(crate)` I thought that only ever meant anything for libraries? 

Thank you for adding me as an author, I''m not 100% sure I want my email public though. Can we change it to `silas.marvin@dmarvin.net` that is an alias for me.', DEFAULT, '2024-06-26 16:02:51', '2024-06-26 16:02:51');
INSERT INTO github.comments VALUES (2174005229, 2357003247, 30024051, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2174005229', 'TODO:

I am not sure how the whole system works, but these two tests works well now.

~~and currently I hard coded the model.~~
', DEFAULT, '2024-06-17 17:59:25', '2024-06-17 18:07:59');
INSERT INTO github.comments VALUES (2271898775, 2426171444, 44469426, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2271898775', 'the problem is copilot does not uses the LS protocol, it uses a proprietary protocol to communicate with the model. ', DEFAULT, '2024-08-06 18:29:48', '2024-08-06 18:29:48');
INSERT INTO github.comments VALUES (2282307842, 2458890504, 25101888, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2282307842', 'RE: A guide that produces a setup for you-- that''d be super neat.

Surely the simplest workflow is to push the LLM setup side of things onto something like Ollama no?

As they''re working on a pretty simple:
1. run this script
2. `ollama pull whatever model you like`
3. `ollama serve whatever model you got`
4. underpants profit?

We can push the more difficult part entirely there in the short term, thereby alleviating the burden of documenting here.

If this is still going on in a few weeks now when I likely next get a chance to write some code I''ll throw in a few PRs, but it looks like your project already has significant interest and someone else will likely pick it up.
', DEFAULT, '2024-08-10 23:07:29', '2024-08-10 23:07:29');
INSERT INTO github.comments VALUES (2161138840, 2343344751, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2161138840', 'If you can make the suggested changes and resolve the merge conflict I will merge it in! Thank you!', DEFAULT, '2024-06-11 16:11:25', '2024-06-11 16:11:25');
INSERT INTO github.comments VALUES (2170030330, 2354890213, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2170030330', 'Thanks for sharing! 

My primary language is English, so I''m not follow everything 100%. Please correct me where I''m wrong:

If I understand correctly, the install when running `cargo install lsp-ai -F llama_cpp` did not work for you, so you created a conda environment, cloned the repo, and then installed it. 

I don''t know what dependencies you will need with Windows to make the install work, but I''m pretty sure cloning the repo didn''t do anything. When you run `cargo install lsp-aI -F llama_cpp` it installs from crates.io. If you want to install it from the local repository you have to run `cargo install --path . -F llama_cpp`. 

I guess I''m confused why you created a conda environment with Python? It seems that it just happened to work because the Conda environment installed some missing libraries, I don''t think that should be required to make the install work. 

Did you look into installing the missing `clang.ddl` and `libclang.dll` libraries, or did you already have them and try setting the `LIBCLANG_PATH` env variable?

If there are some additional steps to getting it working with Windows I would like to add it to the Wiki. Thanks for pointing this out!', DEFAULT, '2024-06-15 15:54:24', '2024-06-15 15:54:24');
INSERT INTO github.comments VALUES (2171919435, 2355993542, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2171919435', 'We do not currently but this sounds like a great option to support! Adding this into the roadmap. I will post some updates here. You are also welcome to add it if you want, it''s not too difficult to add a backend if you have some experience with Rust. I''m happy to point you in the right direction.', DEFAULT, '2024-06-16 22:47:50', '2024-06-16 22:47:50');
INSERT INTO github.comments VALUES (2156400054, 2341830542, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156400054', 'Fixed the Anthropic tool errors, thank you for catching those! https://github.com/SilasMarvin/lsp-ai/pull/7', DEFAULT, '2024-06-09 09:00:10', '2024-06-09 09:00:10');
INSERT INTO github.comments VALUES (2195626962, 2379046522, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2195626962', 'Thanks for flagging this. Is there more to the error message? 

If you want to run local models with an Nvidia GPU I would actually recommend using Ollama. All you need to do is install Ollama and then follow the configuration section of the wiki for setting Ollama up. ', DEFAULT, '2024-06-27 20:37:19', '2024-06-27 20:37:19');
INSERT INTO github.comments VALUES (2479166332, 2537650065, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2479166332', 'If we get something tested and working for a few people I''m happy to merge it in! I''m not very experienced with writing my own flakes though so heavily relying on everyones thoughts here.', DEFAULT, '2024-11-15 15:27:56', '2024-11-15 15:27:56');
INSERT INTO github.comments VALUES (2276892613, 2456334467, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2276892613', 'Oh that is interesting. I don''t know if I have seen that error before. 
 
Try enabling logging as seen in the wiki here: https://github.com/SilasMarvin/lsp-ai/wiki/Debugging
```
export LSP_AI_LOG=DEBUG
```

Before opening Helix, can you clear the log file:
```
rm ~/.cache/helix/helix.log
```

After opening it can you share the log file here. 

Thanks!', DEFAULT, '2024-08-08 23:47:15', '2024-08-08 23:47:15');
INSERT INTO github.comments VALUES (2302470115, 2458890504, 119697612, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2302470115', 'Hey sorry to drop by I don''t know if this is the correct place to voice my problem, but I want to start by thanking you, your project looks amazing and as a hardcore helix-user I''ve been missing on LLM inside helix since they started to become popular, so I''m really excited to get your lsp-working. But currently I''m facing some issues and I''d be really great if someone could guide me or share his configuration for helix.

So currently I''ve installed codellama through this command :
`curl -fsSL https://ollama.com/install.sh | sh`

I''ve decided to use codellama 7b installed everything through the ollama CLI and I was able to run the model locally:
`https://ollama.com/library/codellama` 

Next I''ve configured the lsp in my language.toml as follow

```
[language-server.lsp-ai]
command = "lsp-ai"

[language-server.lsp-ai.config.memory]
file_store = { }

[language-server.lsp-ai.config.models.model1]
type = "ollama"
model = "codellama:7b"

[language-server.lsp-ai.config.completion]
model = "model1"

[language-server.lsp-ai.config.completion.parameters]
max_context = "2000"

[language-server.lsp-ai.config.completion.parameters.option]
num_predicts = "32"
```

And I''ve added that configuration to my language

```
[[language]]
name = "c"
scope = "source.c"
file-types = ["c","h"]
roots = ["Makefile","run.sh","build.zig"]
block-comment-tokens = {start = "/*", end = "*/"}
indent = { tab-width = 8, unit = "\t" }
formatter = { command = "pipx run c_formatter_42", args = [""]}
diagnostic-severity = "error"
auto-format = true
language-servers = ["lsp-ai"]
```

So far so good everything seems about what I would expect from adding an lsp into language.toml appart from the bit of extra configuration. Now I think I must be missing a step because the lsp crashes and fails to complete anything. I''ve tried a bunch of different tweaks but maybe I''m missing something really obvious or maybe the model can''t work with this configuration. I''ve tried to debug with the log_file and even exported the `RUST_BACKTRACE=1` to get more information but all I get when attempting to get a completion request which with my current config is `C-x` is this error :

```
2024-08-21T17:58:45.908 helix_lsp::transport [ERROR] Could not close request on a closed channel (id=Num(1))
2024-08-21T17:58:45.908 helix_lsp::transport [ERROR] Could not close request on a closed channel (id=Num(2))
2024-08-21T17:58:45.908 helix_lsp::transport [ERROR] lsp-ai err: <- StreamClosed
2024-08-21T17:58:45.908 helix_lsp::transport [ERROR] err: <- Other(failed to send a message to server

Caused by:
    channel closed

Stack backtrace:
   0: helix_lsp::transport::Transport::process_server_message::{{closure}}
   1: helix_lsp::transport::Transport::recv::{{closure}}
   2: tokio::runtime::task::core::Core<T,S>::poll
   3: tokio::runtime::task::harness::Harness<T,S>::poll
   4: tokio::runtime::scheduler::multi_thread::worker::Context::run_task
   5: tokio::runtime::scheduler::multi_thread::worker::Context::run
   6: tokio::runtime::context::scoped::Scoped<T>::set
   7: tokio::runtime::context::runtime::enter_runtime
   8: tokio::runtime::scheduler::multi_thread::worker::run
   9: tokio::runtime::task::core::Core<T,S>::poll
  10: tokio::runtime::task::harness::Harness<T,S>::poll
  11: tokio::runtime::blocking::pool::Inner::run
  12: std::sys_common::backtrace::__rust_begin_short_backtrace
  13: core::ops::function::FnOnce::call_once{{vtable.shim}}
  14: std::sys::pal::unix::thread::Thread::new::thread_start
  15: start_thread
             at ./nptl/pthread_create.c:447:8
  16: __GI___clone3
             at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:78)
```

So if you have any solution I''d be glad to hear it, just for information I''m on Ubuntu 24.04, and I''m using `helix 24.7 (0a4432b1)` ', DEFAULT, '2024-08-21 16:13:27', '2024-08-21 16:13:27');
INSERT INTO github.comments VALUES (2479170292, 2537650065, 6033387, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2479170292', 'Update on my side - my bad, actually - that `lsp-ai` has been merged into `nixpkgs` so I guess this PR is obsolete.', DEFAULT, '2024-11-15 15:29:08', '2024-11-15 15:29:08');
INSERT INTO github.comments VALUES (2304175984, 2458890504, 119697612, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2304175984', '[helix.log](https://github.com/user-attachments/files/16708188/helix.log)
I''ve used it to code for 5 minutes on a C++ project and I had multiple expectation of completion but nothing happened, maybe I''m doing something wrong but really nothing happens when I press my usual keybindings for completion. Maybe I''m not doing things correctly I''m not sure. thanks for taking time to look at this 👍🏽 ', DEFAULT, '2024-08-22 09:15:33', '2024-08-22 09:15:45');
INSERT INTO github.comments VALUES (2209487017, 2389880446, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2209487017', 'Right now rayon and simsimd are feature flags that are enabled by default. I have alternative methods for dot products and hamming distance for simsimd but I don''t have an alternative search method that doesn''t  use Rayon. Is that something we should even add? Are we going to have users that don''t want to use Rayon?', DEFAULT, '2024-07-04 19:29:42', '2024-07-04 19:29:42');
INSERT INTO github.comments VALUES (2171271936, 2355576107, 2666479, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2171271936', 'Never mind, I found `file_path` option. I think this should be documented in wiki.

```lua
{
  memory = {
    file_store = {},
  },
  models = {
    model1 =  {
      type="llama_cpp",
      repository= "Qwen/CodeQwen1.5-7B-Chat-GGUF",
      name= "codeqwen-1_5-7b-chat-q4_k_m.gguf",
      file_path="/Users/user1/model/codeqwen-1_5-7b-chat-q4_k_m.gguf",
      n_ctx=2048,
      n_gpu_layers= 1000,
    }
  }
}
```', DEFAULT, '2024-06-16 09:00:20', '2024-06-16 09:00:20');
INSERT INTO github.comments VALUES (2172345469, 2356343380, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2172345469', 'Great issue to flag. This is something we need to add support for. For right now, I would just ignore them, but I will get this on the roadmap. 

Thanks for making an issue for it!', DEFAULT, '2024-06-17 05:53:46', '2024-06-17 05:53:46');
INSERT INTO github.comments VALUES (2158404878, 2343367809, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2158404878', 'This is great! I added it yesterday and honestly didn''t even think about a remote api. I added a few comments in the PR, but otherwise it looks really awesome!', DEFAULT, '2024-06-10 13:37:33', '2024-06-10 13:37:33');
INSERT INTO github.comments VALUES (2161208912, 2343344751, 43348732, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2161208912', 'resolved, [3a351ff](https://github.com/SilasMarvin/lsp-ai/pull/15/commits/3a351ff4a0ed846ceb4bc9d926ac422c8324a8a1)https://github.com/SilasMarvin/lsp-ai/pull/15/commits/237f4cf179b79f74b4ad2e169e8e4f855f96a6b4', DEFAULT, '2024-06-11 16:49:53', '2024-06-11 16:51:13');
INSERT INTO github.comments VALUES (2315403881, 2491879042, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2315403881', 'I''m glad you like it!

LSP-AI is a language server: https://en.wikipedia.org/wiki/Language_Server_Protocol It is designed to be run locally on your computer alongside the editor you are using. You should not host LSP-AI remotely. 

GPUs  become useful when doing text-generation with large LLMs. For this use case, LSP-AI supports making API requests. LSP-AI can make API requests to:
- OpenAI or OpenAI compatible APIs like fireworks, VLLM, etc...
- Anthropic or Anthropic compatible APIs
- Ollama

The API you use is dependent on the model you want to use. The endpoints are all configurable. See: https://github.com/SilasMarvin/lsp-ai/wiki/Configuration

Let me know if you have any other questions. I really do need to improve the general documentation around configurations and how to properly use it.', DEFAULT, '2024-08-28 13:59:46', '2024-08-28 13:59:46');
INSERT INTO github.comments VALUES (2207690516, 2379046522, 55425467, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2207690516', 'Sorry for keeping you waiting. I have now tried to install it without llama_cpp and just run `cargo install lsp-ai -F cuda`. The output of the failed installation is so huge that it takes up the whole history and it is not complete. Do you have a page where I can paste it and send you the link?', DEFAULT, '2024-07-04 00:42:32', '2024-07-04 00:42:32');
INSERT INTO github.comments VALUES (2479252908, 2537650065, 6314611, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2479252908', 'It isn''t obsolete necessary, there might be a desire from a dev perspective to have a nix-ified dev shell. From a consumer environment, yes, the nixpkg version (slightly different to conform to nixpkg standards) is gtg for consumers of the package. I haven''t had a ton of time to test this specifically since I haven''t needed to contribute to the project codebase directly. ', DEFAULT, '2024-11-15 15:41:08', '2024-11-15 15:41:08');
INSERT INTO github.comments VALUES (2283321821, 2456334467, 66112470, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2283321821', 'Hi @SilasMarvin , 

Sorry for delayed response, after set log in DEBUG, it seems worked now on the same project (which have a little bit evolved since my last try).
Very strange.

I''ll keep you posted if I succeed to reproduce.', DEFAULT, '2024-08-12 07:56:51', '2024-08-12 07:56:51');
INSERT INTO github.comments VALUES (2302492069, 2458890504, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2302492069', 'I''m glad you like it! Don''t apologize the documentation is kind of rough right now, I''m happy to help.

Can you set the env variable `LSP_AI_LOG` to `DEBUG`
```bash
export LSP_AI_LOG=DEBUG
```

And then share your helix logs? ', DEFAULT, '2024-08-21 16:23:15', '2024-08-21 16:23:15');
INSERT INTO github.comments VALUES (2241050813, 2420738486, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2241050813', 'You config looks ok. I''m not sure why you are getting that error. Does it happen every time? Is it random?

 Its also worth double checking that you didn''t post a real key here? If that is your auth token please remove it or someone may use it.

I was able to verify that the VS Code extension works with the latest version of LSP-AI. My entire VS Code config:
```
{
  "workbench.colorTheme": "Gruvbox Dark Hard",
  "vim.cursorStylePerMode.normal": "block",
  "editor.quickSuggestions": {
    "other": "inline"
  },
  "extensions.ignoreRecommendations": true,
  "editor.quickSuggestionsDelay": 50,
  "lsp-ai.serverConfiguration": {
    "memory": {
      "file_store": {}
    },
    "models": {
      "model1": {
        "type": "open_ai",
        "chat_endpoint": "https://api.openai.com/v1/chat/completions",
        "model": "gpt-4o",
        "auth_token_env_var_name": "OPENAI_API_KEY"
      }
    }
  },
  "lsp-ai.generationConfiguration": {
    "model": "model1",
    "parameters": {
      "max_tokens": 128,
      "max_context": 1024,
      "messages": [
        {
          "role": "system",
          "content": "You are a programming completion tool. Replace <CURSOR> with the correct code."
        },
        {
          "role": "user",
          "content": "{CODE}"
        }
      ]
    }
  },
  "lsp-ai.inlineCompletionConfiguration": {
    "maxCompletionsPerSecond": 1
  } 
}
```', DEFAULT, '2024-07-20 08:57:19', '2024-07-20 08:57:19');
INSERT INTO github.comments VALUES (2241082035, 2420738486, 66897975, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2241082035', 'The api key is fake though. Thanks for the remind.', DEFAULT, '2024-07-20 10:42:50', '2024-07-20 10:42:50');
INSERT INTO github.comments VALUES (2156225913, 2341830542, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156225913', 'Thanks for checking it out! I think we needed that tool call reference for Mistral FIM, I''m not 100% sure I will double check it tomorrow and get back to you here with a fix.

Can you share your config for Anthropic? You probably need to increase the `max_tokens` parameter.

It''s also worth noting that these LLMs are not deterministic and do have some strange behaviour. We may also need to adjust the prompt you are using. ', DEFAULT, '2024-06-08 23:33:21', '2024-06-08 23:35:02');
INSERT INTO github.comments VALUES (2276132075, 2368077996, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2276132075', 'Fixed with: https://github.com/SilasMarvin/lsp-ai/commit/e3b7aa8186d8bcb7c6f6feaf6c24d9fef7ea0947', DEFAULT, '2024-08-08 15:39:27', '2024-08-08 15:39:27');
INSERT INTO github.comments VALUES (2370993430, 2539087774, 6314611, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2370993430', '@SilasMarvin looks like the tagged commit on `refs/tags/v0.7.0` is not pointing to the newest commit on `releases/v0.7.0`', DEFAULT, '2024-09-24 11:26:04', '2024-09-24 11:26:04');
INSERT INTO github.comments VALUES (2371538109, 2539087774, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2371538109', 'Fixed this with a new release.', DEFAULT, '2024-09-24 14:51:48', '2024-09-24 14:51:48');
INSERT INTO github.comments VALUES (2273586453, 2453150358, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2273586453', 'This is a really good point. This project has an education problem. To use this project effectively you must not only have a decent understanding of LLMs, you must have a good understanding of the LSP. 

This is something I have been thinking about and I have a few ideas. The main one being an auto configurator website where you are asked a series of questions and tick a series of boxes and are given a configuration.

Stay tuned.', DEFAULT, '2024-08-07 14:16:13', '2024-08-07 14:16:13');
INSERT INTO github.comments VALUES (2241783933, 2421318028, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2241783933', 'This is a great idea and easy to add! Using it would look something like:
```json
{
      "role": "user",
      "content": "```{LANGUAGE}\n{CODE}```"
}
```

Where the prompt when expanded would look like:
```json
{
      "role": "user",
      "content": "```python\ndef greet(name):\n    print(f\"Hello, {<CURSOR>}\")\n```"
}
```

One thing we need to think through is the LLM response. If we send it markdown will it respond with markdown? We currently don''t post process LLM responses, but we may need to if we being sending it markdown. In other words, we may want to provide some kind of post process regex response matcher that extracts text so users can specify the code to insert  from the LLM should be the text in the markdown code block.

This extractor would actually be really useful for chain of though prompting: https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering/chain-of-thought#example-writing-donor-emails-structured-guided-cot 

Notice the actual answer for Claude is between `<answer>` and `</answer>` and to let users benefit from chain of thought prompting we would need to allow them to define custom extractors (or have a preset list of extractors like Markdown, etc...).

If you want to take a crack at adding it I would be happy to help make some suggestions, otherwise we can add it to our roadmap.', DEFAULT, '2024-07-21 21:46:10', '2024-07-21 21:47:15');
INSERT INTO github.comments VALUES (2370049215, 2537650065, 6314611, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2370049215', 'Currently working through the nixpkg approval process. It is involved to say the least. I think there might be a difference between the two goals. I mainly just created this flake as a semi easy way to setup a dev environment. I definitely think it can be improved based on the feedback from the nixpkg review process. I think a solution lies somewhere in-between what you have and what I posted', DEFAULT, '2024-09-24 03:38:16', '2024-09-24 03:38:16');
INSERT INTO github.comments VALUES (2304880164, 2458890504, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2304880164', 'I just tested your configuration on my computer and it is working fine for me.

Your logs show a bunch of completion requests to the language server and we can see the language server calling out to ollama:
```txt
2024-08-22T11:07:11.218 helix_lsp::transport [ERROR] lsp-ai err <- " INFO dispatch_request{request=Completion(CompletionRequest { id: RequestId(I32(1)), params: CompletionParams { text_document_position: TextDocumentPositionParams { text_document: TextDocumentIdentifier { uri: Url { scheme: \"file\", cannot_be_a_base: false, username: \"\", password: None, host: None, port: None, path: \"/home/pollivie/workspace/school/CPP-42/CPP03/ex00/ClapTrap.cpp\", query: None, fragment: None } }, position: Position { line: 71, character: 4 } }, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams { partial_result_token: None }, context: Some(CompletionContext { trigger_kind: Invoked, trigger_character: None }) } })}:do_generate{prompt=ContextAndCode(ContextAndCodePrompt { context: \"\", code: \"/* ************************************************************************** */\\n/*                                                                            */\\n/*                                                        :::      ::::::::   */\\n/*   ClapTrap.cpp                                       :+:      :+:    :+:   */\\n/*                                                    +:+ +:+         +:+     */\\n/*   By: pollivie <pollivie.student.42.fr>          +#+  +:+       +#+        */\\n/*                                                +#+#+#+#+#+   +#+           */\\n/*   Created: 2024/08/21 13:30:54 by pollivie          #+#    #+#             */\\n/*   Updated: 2024/08/21 13:30:54 by pollivie         ###   ########.fr       */\\n/*                                                                            */\\n/* ************************************************************************** */\\n\\n#include \\\"ClapTrap.hpp\\\"\\n\\nClapTrap::ClapTrap() {\\n}\\n\\nClapTrap::ClapTrap(std::string name, unsigned int hit_points, unsigned int energy_points, unsigned int attack_damage)\\n    : _name(name), _hit_points(hit_points), _energy_points(energy_points), _attack_damage(attack_damage) {\\n}\\n\\nClapTrap::ClapTrap(ClapTrap const &other) {\\n\\t*this = other;\\n}\\n\\nClapTrap &ClapTrap::operator=(const ClapTrap &other) {\\n\\tif (this != &other) {\\n\\t\\tthis->_name = other._name;\\n\\t\\tthis->_hit_points = other._hit_points;\\n\\t\\tthis->_energy_points = other._energy_points;\\n\\t\\tthis->_attack_damage = other._attack_damage;\\n\\t}\\n\\treturn (*this);\\n}\\n\\nbool ClapTrap::isAlive(void) const {\\n\\treturn (_hit_points != 0);\\n}\\n\\nbool ClapTrap::hasEnoughEnergy(void) const {\\n\\treturn (_energy_points != 0);\\n}\\n\\nvoid ClapTrap::attack(const std::string &target) {\\n\\tstd::cout << \\\"ClapTrap \\\" << _name;\\n\\tif (!isAlive()) {\\n\\t\\tstd::cout << \\\", can''t attack because he is dead\\\";\\n\\t} else if (hasEnoughEnergy()) {\\n\\t\\tstd::cout << \\\" attacks \\\" << target << \\\", causing \\\" << _attack_damage << \\\" points of damage!\\\";\\n\\t\\t_energy_points -= 1;\\n\\t} else {\\n\\t\\tstd::cout << \\\", can''t attack because he lacks energy\\\";\\n\\t}\\n\\tstd::cout << std::endl;\\n}\\n\\nvoid ClapTrap::takeDamage(const unsigned int amount) {\\n\\tstd::cout << \\\"ClapTrap \\\" << _name;\\n\\tif (!isAlive()) {\\n\\t\\tstd::cout << \\\" can''t be attacked because he is dead\\\" << std::endl;\\n\\t} else if (amount >= _hit_points) {\\n\\t\\t_hit_points = 0;\\n\\t\\tstd::cout << \\\" took \\\" << amount << \\\" of damage\\\";\\n\\t\\tstd::cout << \\\" and is now dead\\\" << std::endl;\\n\\t} else {\\n\\t\\t_hit_points -= amount;\\n\\t\\tstd::cout << \\\" took \\\" << amount << \\\" of damage\\\";\\n\\t\\tstd::cout << \\\" and is now left with\\\" << _hit_points << \\\" points of health\\\" << std::endl;\\n\\t}\\n}\\n\\nvoid\", selected_text: None }) params=Object {\"max_context\": String(\"2000\"), \"option\": Object {\"num_predicts\": String(\"32\")}}}: lsp_ai::transformer_backends::ollama: Calling Ollama compatible completions API with parameters:\n"
2024-08-22T11:07:11.218 helix_lsp::transport [ERROR] lsp-ai err <- "{\n"
2024-08-22T11:07:11.218 helix_lsp::transport [ERROR] lsp-ai err <- "  \"keep_alive\": null,\n"
2024-08-22T11:07:11.218 helix_lsp::transport [ERROR] lsp-ai err <- "  \"model\": \"codellama:7b\",\n"
2024-08-22T11:07:11.218 helix_lsp::transport [ERROR] lsp-ai err <- "  \"options\": {},\n"
2024-08-22T11:07:11.218 helix_lsp::transport [ERROR] lsp-ai err <- "  \"prompt\": \"\\n\\n/* ************************************************************************** */\\n/*                                                                            */\\n/*                                                        :::      ::::::::   */\\n/*   ClapTrap.cpp                                       :+:      :+:    :+:   */\\n/*                                                    +:+ +:+         +:+     */\\n/*   By: pollivie <pollivie.student.42.fr>          +#+  +:+       +#+        */\\n/*                                                +#+#+#+#+#+   +#+           */\\n/*   Created: 2024/08/21 13:30:54 by pollivie          #+#    #+#             */\\n/*   Updated: 2024/08/21 13:30:54 by pollivie         ###   ########.fr       */\\n/*                                                                            */\\n/* ************************************************************************** */\\n\\n#include \\\"ClapTrap.hpp\\\"\\n\\nClapTrap::ClapTrap() {\\n}\\n\\nClapTrap::ClapTrap(std::string name, unsigned int hit_points, unsigned int energy_points, unsigned int attack_damage)\\n    : _name(name), _hit_points(hit_points), _energy_points(energy_points), _attack_damage(attack_damage) {\\n}\\n\\nClapTrap::ClapTrap(ClapTrap const &other) {\\n\\t*this = other;\\n}\\n\\nClapTrap &ClapTrap::operator=(const ClapTrap &other) {\\n\\tif (this != &other) {\\n\\t\\tthis->_name = other._name;\\n\\t\\tthis->_hit_points = other._hit_points;\\n\\t\\tthis->_energy_points = other._energy_points;\\n\\t\\tthis->_attack_damage = other._attack_damage;\\n\\t}\\n\\treturn (*this);\\n}\\n\\nbool ClapTrap::isAlive(void) const {\\n\\treturn (_hit_points != 0);\\n}\\n\\nbool ClapTrap::hasEnoughEnergy(void) const {\\n\\treturn (_energy_points != 0);\\n}\\n\\nvoid ClapTrap::attack(const std::string &target) {\\n\\tstd::cout << \\\"ClapTrap \\\" << _name;\\n\\tif (!isAlive()) {\\n\\t\\tstd::cout << \\\", can''t attack because he is dead\\\";\\n\\t} else if (hasEnoughEnergy()) {\\n\\t\\tstd::cout << \\\" attacks \\\" << target << \\\", causing \\\" << _attack_damage << \\\" points of damage!\\\";\\n\\t\\t_energy_points -= 1;\\n\\t} else {\\n\\t\\tstd::cout << \\\", can''t attack because he lacks energy\\\";\\n\\t}\\n\\tstd::cout << std::endl;\\n}\\n\\nvoid ClapTrap::takeDamage(const unsigned int amount) {\\n\\tstd::cout << \\\"ClapTrap \\\" << _name;\\n\\tif (!isAlive()) {\\n\\t\\tstd::cout << \\\" can''t be attacked because he is dead\\\" << std::endl;\\n\\t} else if (amount >= _hit_points) {\\n\\t\\t_hit_points = 0;\\n\\t\\tstd::cout << \\\" took \\\" << amount << \\\" of damage\\\";\\n\\t\\tstd::cout << \\\" and is now dead\\\" << std::endl;\\n\\t} else {\\n\\t\\t_hit_points -= amount;\\n\\t\\tstd::cout << \\\" took \\\" << amount << \\\" of damage\\\";\\n\\t\\tstd::cout << \\\" and is now left with\\\" << _hit_points << \\\" points of health\\\" << std::endl;\\n\\t}\\n}\\n\\nvoid\",\n"
2024-08-22T11:07:11.218 helix_lsp::transport [ERROR] lsp-ai err <- "  \"raw\": true,\n"
2024-08-22T11:07:11.218 helix_lsp::transport [ERROR] lsp-ai err <- "  \"stream\": false\n"
2024-08-22T11:07:11.218 helix_lsp::transport [ERROR] lsp-ai err <- "}\n"
```

What are you typing in your file? Is your file just a ton of asterisks?

The weird thing is I am not seeing any responses from ollama. Have you tested that you can run codellama:7b locally? 

Also make sure to upgrade to the latest version of lsp-ai if you have not:
```
cargo install lsp-ai
```

As it has some better logging features.', DEFAULT, '2024-08-22 14:54:54', '2024-08-22 14:54:54');
INSERT INTO github.comments VALUES (2191732513, 2373729518, 9112841, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2191732513', 'I like the idea!

> cons: increase the cost, the ai will generate different answers each time, the respond speed, etc.

We could cache the responses to make it fast on second hover & keep the result consistent. We could provide a "clear cache" code action in the lsp to clear the generated text if it is unsatisfactory, either global or based on the cursor position to clear the llm-generated content. Could be regenerate rather than clear cache as well (the code action).

@SilasMarvin this would play well with what is stored in the memory backend & could add context to the function by fetching from the vector store.', DEFAULT, '2024-06-26 13:40:57', '2024-06-26 13:41:41');
INSERT INTO github.comments VALUES (2171695047, 2355576107, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2171695047', 'Glad you were able to find it! It is documented under: https://github.com/SilasMarvin/lsp-ai/wiki/Configuration#llamacpp but that configuration page is getting so large it may be worth redoing our docs at some point soon into a small website. I should also note if you are setting the file_path you don''t have to set the repository and name. ', DEFAULT, '2024-06-16 14:43:54', '2024-06-16 14:43:54');
INSERT INTO github.comments VALUES (2160987559, 2346563469, 43348732, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2160987559', 'you might want to run cargo install to install lsp-ai executable to PATH, the vscode plugin doesn''t seem to include valid executable file in windows', DEFAULT, '2024-06-11 15:00:21', '2024-06-11 15:01:55');
INSERT INTO github.comments VALUES (2164260054, 2343367809, 43348732, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2164260054', 'resolved by #15 ', DEFAULT, '2024-06-13 02:55:13', '2024-06-13 02:55:13');
INSERT INTO github.comments VALUES (2315601723, 2492322529, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2315601723', 'I agree this would be nice and really simple to add. Feel free to add it if you want. You would want to edit the main.rs file to take a path name as a command line argument, read it in, and pass it to the Config. Note that if they also pass `initializationOptions` I would say those should override the file if they aren''t empty.

If you don''t want to add it I can probably get to it in the next few weeks. I''m working on an online configurator for LSP-AI that is taking most of my free time.', DEFAULT, '2024-08-28 14:57:16', '2024-08-28 14:57:16');
INSERT INTO github.comments VALUES (2170873881, 2354840741, 2828428, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2170873881', 'It worked, thank you!', DEFAULT, '2024-06-15 21:21:04', '2024-06-15 21:21:04');
INSERT INTO github.comments VALUES (2254225695, 2426171444, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2254225695', 'Can you expand on this please?', DEFAULT, '2024-07-27 18:47:05', '2024-07-27 18:47:05');
INSERT INTO github.comments VALUES (2245562729, 2421318028, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2245562729', '> Not necessarily. In fact, I''ve empirically verified as much by modifying the user messages in the example chat mode prompt in the wiki so that they all contain Markdown code blocks, while leaving the assistant messages untouched. The result: no difference in the generated response than before, i.e. no triple backticks to parse out. So it seems a few in-context example pairs where the answer is ''bare'' sufficiently conditions the final response to also be bare.

Got it thanks for testing that! 

> So by starting the chat completion with a final prefilled assistant message, you get a markdown code block.

Pre-filling the assistant response is a really cool. It is something I would like to support for Anthropic''s API I need to look into the support the other backends have for it. 

> ... Markdown code blocks, the Claude XML tags, ... My thoughts quickly wonder to JSON, but that would call for something different than regex. So maybe let''s restrict the scope to regex and provide presets, or just well-documented examples, of those two.

It might be worth introducing the idea of `Extractors` which are post processors that run over the LLM response and extract a specific part. We can start with two different types: `JSON` and `RegEx`.

We have been discussing the idea of presets in the Discord. There are a few more features on the roadmap first (this now included) and then I want to dial in on presets.

Also just shot you an email would love to connect and talk more on these ideas. I also have a few other ideas I''m looking for feedback from users on.', DEFAULT, '2024-07-23 15:28:12', '2024-07-23 15:28:12');
INSERT INTO github.comments VALUES (2370251307, 2537650065, 180627245, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2370251307', '> That is exactly what I am referring to. I''m a bit worried to merge this in without actually using nix myself and having no way to verify that it is consistently working. I know a lot of people do use it and I would love to have a flake, but I want to make sure it is sustainable.

Since I''ve set up my flake to be mainly for package distribution and this PR set this up to be for the devShell, I could make make a PR to this PR (or directly contribute, whatever is preferred) with parts of my lsp-ai flake and also a GitHub Actions CI for building the nix package if @ProjectInitiative is open to working together!', DEFAULT, '2024-09-24 05:52:01', '2024-09-24 05:52:01');
INSERT INTO github.comments VALUES (2305545542, 2458890504, 119697612, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2305545542', 'Oh I''ve figured it out, I use a school header everywhere in my projects and I think that must confuse the model response or making it a lot slower than it should be, I''m not used to completion taking this long but I guess this make sense on my hardware I only have an rx6700s. So not a top of the line NVIDIA gpu. So I just need to wait for 10s and I actually get a suggestion. Sorry that I bothered you with my logs and thanks for taking the time, I think I''m going to try different models and see if one is snappy enough.', DEFAULT, '2024-08-22 20:10:23', '2024-08-22 20:10:23');
INSERT INTO github.comments VALUES (2192728886, 2373729518, 30024051, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2192728886', 'some other ideas:

hover action add test case for functions.

hover action refactor functions.
', DEFAULT, '2024-06-26 22:33:59', '2024-06-26 22:34:29');
INSERT INTO github.comments VALUES (2161102625, 2346449494, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2161102625', 'Thanks for pointing this out! I just updated the Cargo.lock here: https://github.com/SilasMarvin/lsp-ai/pull/22 It should build with `cargo build --frozen` now.

Would love to have a flake for this if that is something you would want to add!', DEFAULT, '2024-06-11 15:52:49', '2024-06-11 15:53:03');
INSERT INTO github.comments VALUES (2161094228, 2346563469, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2161094228', 'Make sure to install the server before enabling the VS Code plugin: https://github.com/SilasMarvin/lsp-ai/wiki/Installation ', DEFAULT, '2024-06-11 15:48:46', '2024-06-11 15:48:46');
INSERT INTO github.comments VALUES (2220842917, 2379046522, 30024051, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2220842917', '> Sorry for keeping you waiting. I have now tried to install it without llama_cpp and just run `cargo install lsp-ai -F cuda`. The output of the failed installation is so huge that it takes up the whole history and it is not complete. Do you have a page where I can paste it and send you the link?

try &> to a log, then send the file.
or use a pastebin.
', DEFAULT, '2024-07-10 15:34:26', '2024-07-10 15:34:26');
INSERT INTO github.comments VALUES (2161116157, 2345456055, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2161116157', 'Logs can be printed from the server using the `eprintln!()` macro. Where those logs end up is dependent on what editor you are using. In VS Code, those logs end up in the `Output` tab of your terminal, make sure to select the `lsp-ai` dropdown. 

If you want to print logs from the VS Code plugin, you can print them with `console.log` to see these logs, make sure you are are running VS Code in Debug mode and open the developer console. 

If you open the `lsp-ai` repository in VS Code, there is an option in the Debug tab to run in Debug because we defined a `.vscode` directory with some tasks. 

Let me know if this is helpful! You are right, we should add a contributing guide with this info.', DEFAULT, '2024-06-11 15:59:43', '2024-06-11 15:59:43');
INSERT INTO github.comments VALUES (2285161912, 2456334467, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2285161912', 'Glad to hear its working. I''m going to close this but feel free to submit a new issue if something goes wrong. You can also jump into the Discord if you have questions.', DEFAULT, '2024-08-13 01:14:57', '2024-08-13 01:14:57');
INSERT INTO github.comments VALUES (2302495530, 2458890504, 119697612, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2302495530', 'Yes of course this is what I get

```

2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- " INFO lsp_ai: lsp-ai logger initialized starting server\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "DEBUG lsp_server::msg: < {\"jsonrpc\":\"2.0\",\"method\":\"initialize\",\"params\":{\"capabilities\":{\"general\":{\"positionEncodings\":[\"utf-8\",\"utf-32\",\"utf-16\"]},\"textDocument\":{\"codeAction\":{\"codeActionLiteralSupport\":{\"codeActionKind\":{\"valueSet\":[\"\",\"quickfix\",\"refactor\",\"refactor.extract\",\"refactor.inline\",\"refactor.rewrite\",\"source\",\"source.organizeImports\"]}},\"dataSupport\":true,\"disabledSupport\":true,\"isPreferredSupport\":true,\"resolveSupport\":{\"properties\":[\"edit\",\"command\"]}},\"completion\":{\"completionItem\":{\"deprecatedSupport\":true,\"insertReplaceSupport\":true,\"resolveSupport\":{\"properties\":[\"documentation\",\"detail\",\"additionalTextEdits\"]},\"snippetSupport\":true,\"tagSupport\":{\"valueSet\":[1]}},\"completionItemKind\":{}},\"formatting\":{\"dynamicRegistration\":false},\"hover\":{\"contentFormat\":[\"markdown\"]},\"inlayHint\":{\"dynamicRegistration\":false},\"publishDiagnostics\":{\"tagSupport\":{\"valueSet\":[1,2]},\"versionSupport\":true},\"rename\":{\"dynamicRegistration\":false,\"honorsChangeAnnotations\":false,\"prepareSupport\":true},\"signatureHelp\":{\"signatureInformation\":{\"activeParameterSupport\":true,\"documentationFormat\":[\"markdown\"],\"parameterInformation\":{\"labelOffsetSupport\":true}}}},\"window\":{\"workDoneProgress\":true},\"workspace\":{\"applyEdit\":true,\"configuration\":true,\"didChangeConfiguration\":{\"dynamicRegistration\":false},\"didChangeWatchedFiles\":{\"dynamicRegistration\":true,\"relativePatternSupport\":false},\"executeCommand\":{\"dynamicRegistration\":false},\"fileOperations\":{\"didRename\":true,\"willRename\":true},\"inlayHint\":{\"refreshSupport\":false},\"symbol\":{\"dynamicRegistration\":false},\"workspaceEdit\":{\"documentChanges\":true,\"failureHandling\":\"abort\",\"normalizesLineEndings\":false,\"resourceOperations\":[\"create\",\"rename\",\"delete\"]},\"workspaceFolders\":true}},\"clientInfo\":{\"name\":\"helix\",\"version\":\"24.7 (0a4432b1)\"},\"initializationOptions\":{\"completion\":{\"model\":\"model1\",\"parameters\":{\"max_context\":\"2000\",\"option\":{\"num_predicts\":\"32\"}}},\"memory\":{\"file_store\":{}},\"models\":{\"model1\":{\"model\":\"codellama:7b\",\"type\":\"ollama\"}}},\"processId\":198399,\"rootPath\":\"/home/pollivie/workspace/school/archive/dev\",\"rootUri\":\"file:///home/pollivie/workspace/school/archive/dev\",\"workspaceFolders\":[{\"name\":\"dev\",\"uri\":\"file:///home/pollivie/workspace/school/archive/dev\"}]},\"id\":0}    \n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "DEBUG lsp_server::stdio: sending message Request(\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "    Request {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "        id: RequestId(\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "            I32(\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                0,\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "            ),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "        ),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "        method: \"initialize\",\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "        params: Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "            \"capabilities\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                \"general\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"positionEncodings\": Array [\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        String(\"utf-8\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        String(\"utf-32\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        String(\"utf-16\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    ],\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                \"textDocument\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"codeAction\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"codeActionLiteralSupport\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            \"codeActionKind\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                \"valueSet\": Array [\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                    String(\"\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                    String(\"quickfix\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                    String(\"refactor\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                    String(\"refactor.extract\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                    String(\"refactor.inline\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                    String(\"refactor.rewrite\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                    String(\"source\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                    String(\"source.organizeImports\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                ],\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"dataSupport\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"disabledSupport\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"isPreferredSupport\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"resolveSupport\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            \"properties\": Array [\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                String(\"edit\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                String(\"command\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            ],\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"completion\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"completionItem\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            \"deprecatedSupport\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            \"insertReplaceSupport\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            \"resolveSupport\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                \"properties\": Array [\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                    String(\"documentation\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                    String(\"detail\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                    String(\"additionalTextEdits\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                ],\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            \"snippetSupport\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            \"tagSupport\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                \"valueSet\": Array [\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                    Number(1),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                ],\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"completionItemKind\": Object {},\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"formatting\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"dynamicRegistration\": Bool(false),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"hover\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"contentFormat\": Array [\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            String(\"markdown\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        ],\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"inlayHint\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"dynamicRegistration\": Bool(false),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"publishDiagnostics\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"tagSupport\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            \"valueSet\": Array [\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                Number(1),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                Number(2),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            ],\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"versionSupport\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"rename\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"dynamicRegistration\": Bool(false),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"honorsChangeAnnotations\": Bool(false),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"prepareSupport\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"signatureHelp\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"signatureInformation\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            \"activeParameterSupport\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            \"documentationFormat\": Array [\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                String(\"markdown\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            ],\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            \"parameterInformation\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                                \"labelOffsetSupport\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                \"window\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"workDoneProgress\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                \"workspace\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"applyEdit\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"configuration\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"didChangeConfiguration\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"dynamicRegistration\": Bool(false),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"didChangeWatchedFiles\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"dynamicRegistration\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"relativePatternSupport\": Bool(false),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"executeCommand\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"dynamicRegistration\": Bool(false),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"fileOperations\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"didRename\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"willRename\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"inlayHint\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"refreshSupport\": Bool(false),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"symbol\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"dynamicRegistration\": Bool(false),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"workspaceEdit\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"documentChanges\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"failureHandling\": String(\"abort\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"normalizesLineEndings\": Bool(false),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"resourceOperations\": Array [\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            String(\"create\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            String(\"rename\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            String(\"delete\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        ],\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"workspaceFolders\": Bool(true),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "            },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "            \"clientInfo\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                \"name\": String(\"helix\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                \"version\": String(\"24.7 (0a4432b1)\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "            },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "            \"initializationOptions\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                \"completion\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"model\": String(\"model1\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"parameters\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"max_context\": String(\"2000\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"option\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                            \"num_predicts\": String(\"32\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                \"memory\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"file_store\": Object {},\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                \"models\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"model1\": Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"model\": String(\"codellama:7b\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"type\": String(\"ollama\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "            },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "            \"processId\": Number(198399),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "            \"rootPath\": String(\"/home/pollivie/workspace/school/archive/dev\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "            \"rootUri\": String(\"file:///home/pollivie/workspace/school/archive/dev\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "            \"workspaceFolders\": Array [\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                Object {\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"name\": String(\"dev\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"uri\": String(\"file:///home/pollivie/workspace/school/archive/dev\"),\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "                },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "            ],\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "        },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "    },\n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- ")    \n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "DEBUG lsp_server::msg: > {\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"codeActionProvider\":{\"resolveProvider\":true},\"completionProvider\":{},\"textDocumentSync\":2}}}    \n"
2024-08-21T18:24:24.839 helix_lsp::transport [ERROR] lsp-ai err <- "DEBUG lsp_server::msg: < {\"jsonrpc\":\"2.0\",\"method\":\"initialized\",\"params\":{}}    \n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "DEBUG lsp_server::stdio: sending message Notification(\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "    Notification {\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "        method: \"initialized\",\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "        params: Object {},\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "    },\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- ")    \n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "DEBUG lsp_server::msg: < {\"jsonrpc\":\"2.0\",\"method\":\"workspace/didChangeConfiguration\",\"params\":{\"settings\":{\"completion\":{\"model\":\"model1\",\"parameters\":{\"max_context\":\"2000\",\"option\":{\"num_predicts\":\"32\"}}},\"memory\":{\"file_store\":{}},\"models\":{\"model1\":{\"model\":\"codellama:7b\",\"type\":\"ollama\"}}}}}    \n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "DEBUG lsp_server::stdio: sending message Notification(\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "    Notification {\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "        method: \"workspace/didChangeConfiguration\",\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "        params: Object {\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "            \"settings\": Object {\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                \"completion\": Object {\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"model\": String(\"model1\"),\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"parameters\": Object {\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"max_context\": String(\"2000\"),\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"option\": Object {\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                            \"num_predicts\": String(\"32\"),\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                        },\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                },\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                \"memory\": Object {\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"file_store\": Object {},\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                },\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                \"models\": Object {\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                    \"model1\": Object {\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"model\": String(\"codellama:7b\"),\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                        \"type\": String(\"ollama\"),\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                    },\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                },\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "            },\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "        },\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "    },\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- ")    \n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "DEBUG lsp_server::msg: < {\"jsonrpc\":\"2.0\",\"method\":\"textDocument/didOpen\",\"params\":{\"textDocument\":{\"languageId\":\"c\",\"text\":\"/* ************************************************************************** */\\n/*                                                                            */\\n/*                                                        :::      ::::::::   */\\n/*   main.c                                             :+:      :+:    :+:   */\\n/*                                                    +:+ +:+         +:+     */\\n/*   By: pollivie <pollivie.student.42.fr>          +#+  +:+       +#+        */\\n/*                                                +#+#+#+#+#+   +#+           */\\n/*   Created: 2024/07/21 17:50:48 by pollivie          #+#    #+#             */\\n/*   Updated: 2024/07/21 17:50:49 by pollivie         ###   ########.fr       */\\n/*                                                                            */\\n/* ************************************************************************** */\\n\\n#include \\\"slib/include/slib.h\\\"\\n\\nint add(int a, int b){\\n\\treturn (a + b);\\n\\n}\\n\\nint\\tmain(void)\\n{\\n\\tt_smlx_instance *mlx;\\n\\n\\tmlx = smlx_create_instance(720, 1280, \\\"HI\\\");\\n\\tsmlx_clear(mlx, 0x00FF0000);\\n\\tsleep(1);\\n\\tif (mlx)\\n\\t\\tsmlx_destroy_instance(mlx);\\n\\t\\n\\treturn (0);\\n}\\n\",\"uri\":\"file:///home/pollivie/workspace/school/archive/dev/main.c\",\"version\":0}}}    \n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "DEBUG lsp_server::stdio: sending message Notification(\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "    Notification {\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "        method: \"textDocument/didOpen\",\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "        params: Object {\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "            \"textDocument\": Object {\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                \"languageId\": String(\"c\"),\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                \"text\": String(\"/* ************************************************************************** */\\n/*                                                                            */\\n/*                                                        :::      ::::::::   */\\n/*   main.c                                             :+:      :+:    :+:   */\\n/*                                                    +:+ +:+         +:+     */\\n/*   By: pollivie <pollivie.student.42.fr>          +#+  +:+       +#+        */\\n/*                                                +#+#+#+#+#+   +#+           */\\n/*   Created: 2024/07/21 17:50:48 by pollivie          #+#    #+#             */\\n/*   Updated: 2024/07/21 17:50:49 by pollivie         ###   ########.fr       */\\n/*                                                                            */\\n/* ************************************************************************** */\\n\\n#include \\\"slib/include/slib.h\\\"\\n\\nint add(int a, int b){\\n\\treturn (a + b);\\n\\n}\\n\\nint\\tmain(void)\\n{\\n\\tt_smlx_instance *mlx;\\n\\n\\tmlx = smlx_create_instance(720, 1280, \\\"HI\\\");\\n\\tsmlx_clear(mlx, 0x00FF0000);\\n\\tsleep(1);\\n\\tif (mlx)\\n\\t\\tsmlx_destroy_instance(mlx);\\n\\t\\n\\treturn (0);\\n}\\n\"),\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                \"uri\": String(\"file:///home/pollivie/workspace/school/archive/dev/main.c\"),\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "                \"version\": Number(0),\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "            },\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "        },\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- "    },\n"
2024-08-21T18:24:24.840 helix_lsp::transport [ERROR] lsp-ai err <- ")    \n"

```', DEFAULT, '2024-08-21 16:24:51', '2024-08-21 16:24:51');
INSERT INTO github.comments VALUES (2241081925, 2420738486, 66897975, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2241081925', 'yeah, I found the cause. I have another extension which got the same functionality as lsp-ai. When I disabled it, it returned.
So I close this issue. 

But I got some insteresting and I would like to share. I opened a rust project then I create a python file in the same vscode.
When I play the example code, I would expect fib function. But I got some unexpected rust code part of which definitely is from the rust project. BTW, the rust project is lsp-ai. You can find the result as the screenshot below:

![Screenshot 2024-07-20 at 6 22 53 PM](https://github.com/user-attachments/assets/f3ffaa69-1b2b-4a0c-a0cb-bd44d265f100)

and this one:
![Screenshot 2024-07-20 at 6 38 29 PM](https://github.com/user-attachments/assets/87cc7d74-ac90-4dbe-9d6d-427f7d0e43b2)

The file is a python file.
But when I just tell explicitly in the comment using python. Then the result is fine. 

![Screenshot 2024-07-20 at 6 40 49 PM](https://github.com/user-attachments/assets/c799c24b-1a79-4543-9068-a9495b213618)

Nice.
', DEFAULT, '2024-07-20 10:42:16', '2024-07-20 10:42:16');
INSERT INTO github.comments VALUES (2192595157, 2374752627, 9112841, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2192595157', '> Wow this is really incredible thank you for adding all this! I am not very familiar with GitHub actions and have learned a lot reading through this.

There are most likely still some things we can do to cache builds but I don''t think it''s necessary on a release CI given it should be done seldomly! Glad you learned something 😄 
We could also add a CI to run the test suite on every push (caching would definitely be interesting here).

> I''m not 100% sure why we changed pub to pub(crate) I thought that only ever meant anything for libraries?

There will be warnings on some platforms and it''ll cause errors in the build. We could disable the visibility warnings but I think they can be useful at times, so I''d rather set the visibility to `pub(crate)`.
Even though it is a binary, I believe visibility is determined at the module level regardless of what the target output is, lib or bin (don''t quote me on that I need to check).

> Thank you for adding me as an author, I''m not 100% sure I want my email public though. Can we change it to silas.marvin@dmarvin.net that is an alias for me.

Sorry about that, changed the commit and force pushed, it shouldn''t be accessible anymore.', DEFAULT, '2024-06-26 20:42:45', '2024-06-26 20:42:45');
INSERT INTO github.comments VALUES (2221356254, 2379046522, 55425467, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2221356254', 'I put everything that was possible in this pastebin: https://pastebin.com/wUfRB16N', DEFAULT, '2024-07-10 20:14:40', '2024-07-27 10:18:00');
INSERT INTO github.comments VALUES (2156226904, 2341844712, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156226904', 'Oh yes absolutely we will! Adding this as an enhancement on the roadmap. Thank you for the suggestion!', DEFAULT, '2024-06-08 23:38:21', '2024-06-08 23:38:21');
INSERT INTO github.comments VALUES (2161131791, 2346449494, 38045210, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2161131791', 'Sure, I can make a PR when I have a flake ready. ', DEFAULT, '2024-06-11 16:07:41', '2024-06-11 16:07:41');
INSERT INTO github.comments VALUES (2159792209, 2345377798, 71929976, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2159792209', 'After few tries , lsp-ai starts working but completions are not shown, in server logs (rpc)
I get like this(after a long delay)

```
.....
// Receive:
{"jsonrpc":"2.0","id":11,"result":{"isIncomplete":false,"items":[{"filterText":"def a","kind":1,"label":"ai - b: int ->int(x,y)\n\nreturn x + y\n\n\nfunction c <| fim begin ||> d  |<|{print(\"Hello World","textEdit":{"newText":"b: int ->int(x,y)\n\nreturn x + y\n\n\nfunction c <| fim begin ||> d  |<|{print(\"Hello World","range":{"end":{"character":5,"line":1},"start":{"character":5,"line":1}}}}]}}
// Receive:
{"jsonrpc":"2.0","id":13,"result":{"isIncomplete":false,"items":[{"filterText":"def add","kind":1,"label":"ai -  (a: int, b :int) ->  | fim begin | >result < ｜end match result do if true then return(add_(1","textEdit":{"newText":" (a: int, b :int) ->  | fim begin | >result < ｜end match result do if true then return(add_(1","range":{"end":{"character":7,"line":1},"start":{"character":7,"line":1}}}}]}}
```

Seems its working and I guess its editor problem', DEFAULT, '2024-06-11 04:55:16', '2024-06-11 04:55:16');
INSERT INTO github.comments VALUES (2275919308, 2379046522, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2275919308', 'This looks like a cuda compilation error. We don''t deal with cuda directly instead we use the https://github.com/utilityai/llama-cpp-rs crate. I''m going to close this as its not anything we can change. I do encourage you to open up a issue on their github.', DEFAULT, '2024-08-08 14:04:41', '2024-08-08 14:04:41');
INSERT INTO github.comments VALUES (2156637438, 2342192769, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156637438', 'This sounds like it would be a great addition. I will add Ollama support today. Stay tuned!', DEFAULT, '2024-06-09 14:45:55', '2024-06-09 14:45:55');
INSERT INTO github.comments VALUES (2156194111, 2341859996, 941359, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156194111', 'Just found them in the `examples/` directory. Sorry.', DEFAULT, '2024-06-08 21:37:46', '2024-06-08 21:37:46');
INSERT INTO github.comments VALUES (2161118575, 2345377798, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2161118575', 'Thanks for creating an issue! I haven''t tested it in Zed yet. Can you share your configuration for Zed so I can try to recreate it and fix these bugs? ', DEFAULT, '2024-06-11 16:00:55', '2024-06-11 16:00:55');
INSERT INTO github.comments VALUES (2163803525, 2343344751, 43348732, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2163803525', '> > Is github not working as intended? i cannot see them on my side, nor my mailbox, i will make the change shown in your screenshot ![image](https://private-user-images.githubusercontent.com/43348732/339102890-04cd4287-1f31-4ac9-810b-d72067fe8345.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MTgyMjEzNzYsIm5iZiI6MTcxODIyMTA3NiwicGF0aCI6Ii80MzM0ODczMi8zMzkxMDI4OTAtMDRjZDQyODctMWYzMS00YWM5LTgxMGItZDcyMDY3ZmU4MzQ1LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNDA2MTIlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjQwNjEyVDE5Mzc1NlomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWJmOGVkMDQ0ZDJjNzMzNmE1MDM5YzU5NjE4ZTUwM2FlZmNiY2UzMzI2MTYzMTZiY2M5NTY0NDI4YTRjODJiNzkmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JmFjdG9yX2lkPTAma2V5X2lkPTAmcmVwb19pZD0wIn0.JmilMPS6t1nv0t2GHHYQpfgesudDw82Y439Zna9fK-8)
> > > I liked the defaults you had before.
> > 
> > 
> > i will revert them at next commit
> > `suggested by gpt-4o: try "submitreview"`
> 
> Oh super strange. I have attached the two other suggestions that I made.
> 
> <img alt="Screenshot 2024-06-12 at 12 38 12 PM" width="875" src="https://private-user-images.githubusercontent.com/19626586/339107571-6aed466d-90b1-4e44-8fed-1ea2c3ecd02b.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MTgyMjI2ODgsIm5iZiI6MTcxODIyMjM4OCwicGF0aCI6Ii8xOTYyNjU4Ni8zMzkxMDc1NzEtNmFlZDQ2NmQtOTBiMS00ZTQ0LThmZWQtMWVhMmMzZWNkMDJiLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNDA2MTIlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjQwNjEyVDE5NTk0OFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTUyODdhNTNjYmFmNmFmZTkxYzc5YzRlY2JkZDE3NGFmZmRkNDE0ZDA5MmMyYjM0ODZhNTUzOTlmYmM0NmY4MjImWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JmFjdG9yX2lkPTAma2V5X2lkPTAmcmVwb19pZD0wIn0.fQmvQHmp_ezEO3Pmrvt63qNzmCjFZRRCHaus6zsOI64"> <img alt="Screenshot 2024-06-12 at 12 38 16 PM" width="879" src="https://private-user-images.githubusercontent.com/19626586/339107593-432dd425-cfac-4d55-b740-67296e922914.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MTgyMjI2ODgsIm5iZiI6MTcxODIyMjM4OCwicGF0aCI6Ii8xOTYyNjU4Ni8zMzkxMDc1OTMtNDMyZGQ0MjUtY2ZhYy00ZDU1LWI3NDAtNjcyOTZlOTIyOTE0LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNDA2MTIlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjQwNjEyVDE5NTk0OFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTdhMjAwMDc5MWQ0OTNiZDIxM2Q1NTAxOWMzZGMzODA0M2UxYWJjNTUxOWNjNmQ2ZjllNDk1Y2ZlNTZlMDQ4ZDAmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JmFjdG9yX2lkPTAma2V5X2lkPTAmcmVwb19pZD0wIn0.QXU4sF70TqBkei1auv6dzOq_oS6qajvJteyrmmcO4Z0">

I''ve addressed the suggested modifications and it''s ready for your feedback', DEFAULT, '2024-06-12 20:01:57', '2024-06-12 20:01:57');
INSERT INTO github.comments VALUES (2161224717, 2345377798, 71929976, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2161224717', '> Thanks for creating an issue! I haven''t tested it in Zed yet. Can you share your configuration for Zed so I can try to recreate it and fix these bugs?

Here is extension source: https://github.com/bajrangCoder/zed-lsp-ai

> We can''t configure any unknown lsp directly, it should be added through extension in zed. But we can configure lsp options from settings', DEFAULT, '2024-06-11 16:58:13', '2024-06-11 17:02:17');
INSERT INTO github.comments VALUES (2158402790, 2343344751, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2158402790', 'This is really awesome thank you!

What do you think about explicitly adding a `chat_endpoint` and  a `generate_endpoint` in the config options for Ollama instead of just `endpoint`? We do this for every other API backend. If a user uses Ollama remotely are we sure they will have chat at `/api/chat` and generate at `/api/generate`?', DEFAULT, '2024-06-10 13:36:34', '2024-06-10 13:36:57');
INSERT INTO github.comments VALUES (2163840072, 2343344751, 43348732, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2163840072', 'my bad, it''s done, never heard of deref before', DEFAULT, '2024-06-12 20:26:35', '2024-06-12 20:26:57');
INSERT INTO github.comments VALUES (2161877077, 2341830542, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2161877077', 'Ah got it! Sorry about that, just updated the docs!', DEFAULT, '2024-06-12 01:14:16', '2024-06-12 01:14:16');
INSERT INTO github.comments VALUES (2158122772, 2341547917, 304428, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2158122772', 'nvim-cmp just merged support for multi-line ghost text, so a cmp based setup should now be quite viable. I''ll play with it some more and see if I can get a decent example config going.

Update: yeah that works, the main issue right now is that the window containing the completion is drawn below the cursor which hides the ghost text on the following lines, but there''s a PR (#1955) addressing it. I''ll play with that branch a bit.
Update 2: yeah it''s not perfect, the windows does not _always_ go above the cursor, but it kinda works. The first character of the prediction is also not displayed in ghost text, not sure if a config problem or a cmp bug. Anyway, it looks like this: 
![Screenshot_20240610_145120](https://github.com/SilasMarvin/lsp-ai/assets/304428/ef3c9e47-98d8-48bc-a2ac-9c3980d98cf6)
', DEFAULT, '2024-06-10 11:44:51', '2024-06-10 12:53:24');
INSERT INTO github.comments VALUES (2169701777, 2354815766, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2169701777', 'Sorry I don''t think I fully understand what is going on. Are you showing output from LSP-AI here? It might be helpful to take a video or breakdown step by step what is going on.

Also note that you can play around with prompts if you don''t like the output you are getting. If you want to code in just Python, it may be worth adjusting the few shot prompts to be only in Python. These could be some fun experiments worth testing.', DEFAULT, '2024-06-15 13:57:56', '2024-06-15 13:57:56');
INSERT INTO github.comments VALUES (2201724907, 2343972535, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2201724907', '> Just a heads up that I haven''t forgotten about this, just got a bit busy lately plus my ISP has issues so no internet for me right now. I should be able to resume this weekend or next week.

Thanks for the update! Sounds good', DEFAULT, '2024-07-02 02:41:59', '2024-07-02 02:41:59');
INSERT INTO github.comments VALUES (2275930395, 2354890213, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2275930395', 'I''m going to close this. It seems like an issue compiling llama.cpp on Windows which means its a https://github.com/utilityai/llama-cpp-rs not us bug. If you still have issues I recommend opening an issue on their GitHub page', DEFAULT, '2024-08-08 14:09:38', '2024-08-08 14:09:38');
INSERT INTO github.comments VALUES (2175063817, 2357003247, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2175063817', '> ok, find this problem: openai use this
> 
> ```rust
> #[derive(Debug, Clone, Deserialize, Serialize)]
> #[serde(deny_unknown_fields)]
> pub struct ChatMessage {
>     pub role: String,
>     pub content: String,
> }
> ```
> 
> but gemini use
> 
> ```json
> "contents": [
>               {
>                 "role":"user",
>                 "parts":[{
>                  "text": "Pretend you''re a snowman and stay in character for each response."}]
>                 },
>               {
>                 "role": "model",
>                 "parts":[{
>                  "text": "Hello! It''s so cold! Isn''t that great?"}]
>                 },
>               {
>                 "role": "user",
>                 "parts":[{
>                  "text": "What''s your favorite season of the year?"}]
>                 }
>              ]
> ```
> 
> the chat functionality seems need to change the system, so support completion first.

I would create a new struct `GeminiChatMessage` and implement `From<ChatMessage>`. It would look something like:

```
#[derive(Serialize)]
struct Part {
    text: String
}

#[derive(Serialize)]
struct GeminiChatMessage {
    role: String,
    parts: Vec<Part>
}

impl From<ChatMessage> for GeminiChatMessage {
    fn from(value: ChatMessage) -> Self {
        // Finish from code here
    }
}
```

Let me know if this isn''t helpful, and I can make a PR into your branch with some suggestions.', DEFAULT, '2024-06-18 05:47:29', '2024-06-18 05:48:00');
INSERT INTO github.comments VALUES (2156643414, 2342117668, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156643414', 'https://github.com/SilasMarvin/lsp-ai/pull/10', DEFAULT, '2024-06-09 15:07:01', '2024-06-09 15:07:01');
INSERT INTO github.comments VALUES (2163127389, 2345377798, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2163127389', 'There may be a rather large quality loss when converting it to gguf. I was testing it outside of LSP-AI using Ollama''s python library directly. 

Thanks for creating the issue! Excited to see what the zed team says,', DEFAULT, '2024-06-12 14:14:50', '2024-06-12 14:15:18');
INSERT INTO github.comments VALUES (2163193125, 2343344751, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2163193125', '> Sorry, I misunderstood your suggestion. The suggested changes have been committed.

Don''t apologize you are totally fine. I think we are having some issues communicating. If you scroll to the top of the page, you will see I have added some suggestions for changes. You can click commit on those directly, or you can add them yourself. Please verify they work and I will merge the PR. Thank you for your contributions!', DEFAULT, '2024-06-12 14:40:39', '2024-06-12 14:40:39');
INSERT INTO github.comments VALUES (2156635877, 2341830542, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156635877', 'Can you share your config?', DEFAULT, '2024-06-09 14:40:49', '2024-06-09 14:40:49');
INSERT INTO github.comments VALUES (2213409001, 2389880446, 9112841, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2213409001', '> Are we going to have users that don''t want to use Rayon?

I think "imposing" rayon is fine. You can always set a config flag that limits the max concurrency. Same goes for simd imo.', DEFAULT, '2024-07-08 08:47:55', '2024-07-08 08:47:55');
INSERT INTO github.comments VALUES (2156815534, 2342506239, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156815534', 'I just tried RSing VScode and I see this now.

`ERROR lsp_ai::transformer_worker: generating response: environment variable not found`

<img width="2056" alt="Screenshot 2024-06-09 at 6 20 47 PM" src="https://github.com/SilasMarvin/lsp-ai/assets/16382165/d3f107ba-c254-4bac-adbc-45d3700c302f">


If I''m not mistaken I didn''t need one if Im using local llama?', DEFAULT, '2024-06-09 22:21:02', '2024-06-09 22:21:47');
INSERT INTO github.comments VALUES (2171019977, 2354815766, 74125308, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2171019977', '![image](https://github.com/SilasMarvin/lsp-ai/assets/74125308/b51e7d7e-cd89-44d7-8fc9-df3d149f5291)
In the code generated using LSP AI, there will be a markdown code syntax as shown in the figure, but I only want to keep the code, and other code syntax does not need to be displayed in cursor.', DEFAULT, '2024-06-16 02:42:11', '2024-06-16 02:42:11');
INSERT INTO github.comments VALUES (2401968803, 2343972535, 304428, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2401968803', 'Well that took a while (ISP troubles dragged on a bit then this kinda fell off my radar, apologies) but I finally got around to it and got a basic config merged in nvim-lspconfig, I see you''ve added an example config, I''ll go and update that.', DEFAULT, '2024-10-09 10:45:17', '2024-10-09 10:45:17');
INSERT INTO github.comments VALUES (2175800905, 2357003247, 30024051, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2175800905', 'now it works well.

```
[src/transformer_backends/gemini.rs:292:9] &response.generated_text = "\"Oh, definitely winter! I mean, where else would a snowman like me be happy? It''s a time for snowballs and sledding, and best of all, making new friends. Like you! 😄 \\n\""
test transformer_backends::gemini::test::gemini_chat_do_generate ... ok
[src/transformer_backends/gemini.rs:259:9] response.generated_text = "\"```python\\ndef test_context():\\n    \\\"\\\"\\\"This function is a placeholder for testing the context.\\n\\n    You can add your own assertions and logic here to test specific aspects of the context.\\n    \\\"\\\"\\\"\\n    pass\\n\\ndef test_code():\\n    \\\"\\\"\\\"This function is a placeholder for testing your code.\\n\\n    You can add your own assertions and logic here to test the functionality of your code.\\n    \\\"\\\"\\\"\\n    # Example:\\n    assert 1 + 1 == 2, \\\"Basic arithmetic should work.\\\"\\n```\\n\\n**Explanation:**\\n\\n1. **`test_context()`:**\\n   - This function is designed to test the context within which your code will run.\\n   - You can use this to check:\\n     - **Environment variables:**  Are the necessary environment variables set correctly?\\n     - **Dependencies:** Are all the required libraries installed and accessible?\\n     - **Data:** Is the data (if any) loaded properly and in the expected format?\\n\\n2. **`test_code()`:**\\n   - This function is for testing the core logic of your code.\\n   - It should include:\\n     - **Assertions:** Use the `assert` keyword to verify that your code produces the expected output.\\n     - **Example:** The provided example `assert 1 + 1 == 2, \\\"Basic arithmetic should work.\\\"` demonstrates how to use an assertion.\\n\\n**Example Usage (within a test file):**\\n\\n```python\\nimport unittest\\n\\nclass MyTests(unittest.TestCase):\\n    def test_context(self):\\n        # Add your assertions about the context here\\n        # ...\\n\\n    def test_code(self):\\n        # Add your code tests here\\n        # ...\\n\\nif __name__ == ''__main__'':\\n    unittest.main()\\n```\\n\\n**To Run Tests:**\\n\\n1. **Save the code** in a file (e.g., `test_my_code.py`).\\n2. **Execute the tests:**\\n   - From your terminal, run: `python -m unittest test_my_code.py`\\n\\n**Remember:**\\n\\n- Replace the placeholder comments with your actual test logic.\\n- Tailor the tests to cover all the functionality of your code.\\n- Use descriptive test names that clearly indicate what is being tested. \\n\""
test transformer_backends::gemini::test::gemini_completion_do_generate ... ok
```

with

```
cargo test -- --nocapture
```
', DEFAULT, '2024-06-18 10:49:12', '2024-06-18 10:49:12');
INSERT INTO github.comments VALUES (2177317588, 2357003247, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2177317588', 'I actually made a pr into your branch: https://github.com/asukaminato0721/lsp-ai/pull/1

If you can merge that I should be able to merge this in.', DEFAULT, '2024-06-19 00:50:17', '2024-06-19 00:50:17');
INSERT INTO github.comments VALUES (2254664557, 2434119177, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2254664557', 'There is a section on the wiki for debugging but basically just set the env variable `LSP_AI_LOG` to `DEBUG`:
```bash
export LSP_AI_LOG=DEBUG
```

Logs for helix typically go in the `~/.cache/helix/helix.log` file. You should see error messages there.', DEFAULT, '2024-07-28 22:06:13', '2024-07-28 22:06:13');
INSERT INTO github.comments VALUES (2252118209, 2345377798, 479816, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2252118209', 'I noticed that Zed does already support Ollama and a few other model execution frameworks/services: https://github.com/zed-industries/zed/tree/main/crates/language_model/src/provider

I understand that lsp-ai has a different range of supported frameworks and is flexible in different ways, so it''s still exciting to see a proper Zed extension, but some users may find it is already not necessary for their use cases', DEFAULT, '2024-07-26 07:11:34', '2024-07-26 07:11:34');
INSERT INTO github.comments VALUES (2163735768, 2343344751, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2163735768', 'Thanks for testing it locally. These are the suggestions I am talking about:

![Screenshot 2024-06-12 at 12 14 49 PM](https://github.com/SilasMarvin/lsp-ai/assets/19626586/41a5d8aa-dc4e-4a37-8a6d-29e639043b24)

We shouldn''t call it `completions_endpoint` as Ollama calls it generate. I liked the defaults you had before.

If you can commit those suggestions and test it, I will merge it!
', DEFAULT, '2024-06-12 19:16:45', '2024-06-12 19:16:45');
INSERT INTO github.comments VALUES (2156810275, 2342506239, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156810275', '![todo](https://github.com/SilasMarvin/lsp-ai/assets/16382165/5c921714-3b7d-45fa-ab47-bdc1e8f84964)


<img width="2056" alt="Screenshot 2024-06-09 at 6 05 37 PM" src="https://github.com/SilasMarvin/lsp-ai/assets/16382165/497e7d12-2aef-4ffb-89d7-fb476ee86b13">


```json
"lsp-ai.serverConfiguration": {
    "memory": {
      "file_store": {}
    },
    "models": {
      "model1": {
        "type": "llama_cpp",
        "repository": "stabilityai/stable-code-3b",
        "name": "stable-code-3b-Q5_K_M.gguf",
        "n_ctx": 2048
      }
    }
  },
  "lsp-ai.generationConfiguration": {
    "model": "model1",
    "parameters": {
      "fim": {
        "start": "<fim_prefix>",
        "middle": "<fim_suffix>",
        "end": "<fim_middle>"
      },
      "max_context": 2000,
      "max_new_tokens": 32
    }
  },
  "lsp-ai.inlineCompletionConfiguration": {
    "maxCompletionsPerSecond": 1
  },
```

Hope this helps. I think it''s working cause I see a line added but there''s no other indicator.

Might it be because I''m on MacOS and I used metal with my install? There isn''t any example config for llama with metal so not sure what to put where. Thanks again.', DEFAULT, '2024-06-09 22:06:24', '2024-06-09 22:08:58');
INSERT INTO github.comments VALUES (2156818032, 2342506239, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156818032', '> auth_token
That was it. Thanks~!
', DEFAULT, '2024-06-09 22:29:23', '2024-06-09 22:29:23');
INSERT INTO github.comments VALUES (2171919842, 2354815766, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2171919842', 'This is a mixture of not the best prompt, and a limitation of LLMs. Its really difficult to get them to do code completion correctly, and that prompt that I came up with is definitely not the best. You are welcome to play around with the prompt, or try a FIM model like Codestral', DEFAULT, '2024-06-16 22:49:33', '2024-06-16 22:49:33');
INSERT INTO github.comments VALUES (2408218707, 2343972535, 304428, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2408218707', 'OK I''ve been playing a bit with lsp-ai in nvim the last couple of days, I think the base config works so I''d rather not hold this much longer (lol it''s been 3 months), but here''s a few notes from my experiments:

* The `nvim-lspconfig` default configuration I''ve contributed attaches the language server to every filetype because of the generic nature of an AI-powered language server. This however, causes unexpected behavior when the language server attaches to buffers such as terminal windows and other windows from plugins and attempting inline completions there but not being able to read physical files (my LSP log files are full of errors caused by terminal windows for instance). I don''t have a perfect solution so far, so I''ve mentioned the issue in a comment, restricting `lsp-ai` to the filetypes you use is probably the easiest solution in this case.
* Also, the default config runs in single-file mode to work around not setting a `root_dir` by default , leaving it to the user for the sake of being generic. Some sort of default might make sense, like detecting a git repo and using that, the nvim-lspconfig contributor guide suggested some APIs but it seems like I don''t have them on my older nvim 0.9.x, so I''ll leave that as future work for the moment I guess.

Now, from my limited understanding of the LSP and what I''ve seen in the lsp-ai docs, the file store is still the only supported backend, and the prompt docs suggest no context is injected when using the file store as a backend, so no `root_dir` being set should not matter *for now* if I understand it correctly ? Docs should probably be expanded when that is no longer the case though.', DEFAULT, '2024-10-11 23:01:13', '2024-10-11 23:03:15');
INSERT INTO github.comments VALUES (2182130282, 2357003247, 30024051, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2182130282', 'cc @SilasMarvin mind take a look？', DEFAULT, '2024-06-21 07:00:38', '2024-06-21 07:00:38');
INSERT INTO github.comments VALUES (2178983485, 2357003247, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2178983485', 'Hey sorry I tested it with my editor and had to add a few more updates to get the prompt to format correctly: https://github.com/asukaminato0721/lsp-ai/pull/2

I''m not able to get good completion responses from it. When I test it using the default chat prompts I have outlined in the Configuration section it loves to include quotes in its response. We can merge this once you merge my pr in, but before I update the wiki it would be nice to have some prompts we know work well.', DEFAULT, '2024-06-19 15:29:49', '2024-06-19 15:29:49');
INSERT INTO github.comments VALUES (2255989413, 2434119177, 94681915, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2255989413', 'Great thank you ! The logs seem to flag every thing as error even they aren''t but I was able to fix the issue', DEFAULT, '2024-07-29 13:44:06', '2024-07-29 13:44:06');
INSERT INTO github.comments VALUES (2252221362, 2345377798, 71929976, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2252221362', '> I noticed that Zed does already support Ollama and a few other model execution frameworks/services: https://github.com/zed-industries/zed/tree/main/crates/language_model/src/provider

That''s code assistant and that''s like a chat or some ai features that built inside editor that''s not like copilot ', DEFAULT, '2024-07-26 08:19:35', '2024-07-26 08:19:35');
INSERT INTO github.comments VALUES (2163748039, 2343344751, 43348732, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2163748039', 'Is github not working as intended? i cannot see them on my side, nor my mailbox, i will make the change shown in your screenshot
![image](https://github.com/SilasMarvin/lsp-ai/assets/43348732/04cd4287-1f31-4ac9-810b-d72067fe8345)

> I liked the defaults you had before.

i will revert them at next commit

`suggested by gpt-4o:  try "submitreview"`', DEFAULT, '2024-06-12 19:24:48', '2024-06-12 19:33:15');
INSERT INTO github.comments VALUES (2161124158, 2341830542, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2161124158', 'Your `max_new_tokens` param should just be `max_tokens`. That should fix your problem! By default we set a `max_tokens` of 64, so right now with your current configuration, you are only able to generate up to 64 tokens.

Did you get this config from the docs somewhere? If so, I need to go update it, I apologize for the bad documentation.', DEFAULT, '2024-06-11 16:03:42', '2024-06-11 16:03:42');
INSERT INTO github.comments VALUES (2156813883, 2342506239, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156813883', 'I''ve been wanting to find a os project to work on for some time now but hadn''t had any luck. Felt like all other apps have been handled. I hope to be able to contribute to this one since it''s so early and I''m sure you could use help and have other ideas. I haven''t spun up an electron app is years though, lol. Also I know little about ML and even less about Rust so yeah... appreciate your work, looking forward to this becoming a "have to have" app on VSCode.', DEFAULT, '2024-06-09 22:16:18', '2024-06-09 22:16:18');
INSERT INTO github.comments VALUES (2175337141, 2354890213, 162593633, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2175337141', 'Salut, oui, j''ai suivi la procédure. J''ai remarqué que dans les fichiers d''installation des dépendances (requirements, etc...), les dépendances qu''on installe,  propose souvent la version ^= et c''est souvent là que je dois corriger en == pour éviter les problèmes.
Exemple dans d''autre projet : "numpy" est passé en version  2 récemment, il provoque des bugs d''installation, il faut repasser en version 1. pour réussir l''installation.  
Bonne journée à vous...', DEFAULT, '2024-06-18 07:12:00', '2024-06-18 07:12:00');
INSERT INTO github.comments VALUES (2174881814, 2357003247, 30024051, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2174881814', 'ok, find this problem: openai use this

```rs
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}
```

but gemini use

```json
"contents": [
              {
                "role":"user",
                "parts":[{
                 "text": "Pretend you''re a snowman and stay in character for each response."}]
                },
              {
                "role": "model",
                "parts":[{
                 "text": "Hello! It''s so cold! Isn''t that great?"}]
                },
              {
                "role": "user",
                "parts":[{
                 "text": "What''s your favorite season of the year?"}]
                }
             ]
```

the chat functionality seems need to change the system, so support completion first.
', DEFAULT, '2024-06-18 03:11:21', '2024-06-18 03:11:21');
INSERT INTO github.comments VALUES (2156688690, 2342192769, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156688690', 'Added Ollama here! https://github.com/SilasMarvin/lsp-ai/pull/11 Pushing out a new release now', DEFAULT, '2024-06-09 16:28:46', '2024-06-09 16:28:46');
INSERT INTO github.comments VALUES (2162158955, 2345377798, 71929976, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2162158955', '> but Zed doesn''t always show them


Does Zed show completions sometimes, as you mentioned, **but not always** ? 😳

I had already share this in their discord, btw I will create a issue for it on their GitHub.

> Also, how do you find codegemma''s results?.......

Currently, Ollama fails to install any model on my system. Previously, I had Codestral and Llama3, but when I tried to install a new model, I faced some issues. I thought it was a problem with my system, so I reinstalled Ollama. Now, I have lost all installed models.

Previously, I downloaded the Codegemma binary (.gguf) just for testing and added it locally to try out LSP AI. This is why I am compelled to use Codegemma.(Yep, it''s not good for coding even its name suggests)', DEFAULT, '2024-06-12 05:51:45', '2024-06-12 05:52:04');
INSERT INTO github.comments VALUES (2162142776, 2343344751, 43348732, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2162142776', 'Sorry, I misunderstood your suggestion. The suggested changes have been committed.', DEFAULT, '2024-06-12 05:35:25', '2024-06-12 05:35:25');
INSERT INTO github.comments VALUES (2156439936, 2341830542, 68561, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156439936', 'Hm… I suppose not.

When I started helix with the old setting (at 2048 max_tokens) it didn''t work.
I then changed the config to 4096 and did a :config-reload and a :lsp-restart

Then it worked.

Now after restarting the editor it returned the cropped results again and I can''t reproduce the above steps.', DEFAULT, '2024-06-09 10:49:38', '2024-06-09 10:50:22');
INSERT INTO github.comments VALUES (2276131759, 2451215027, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2276131759', 'Closed with: https://github.com/SilasMarvin/lsp-ai/commit/e3b7aa8186d8bcb7c6f6feaf6c24d9fef7ea0947', DEFAULT, '2024-08-08 15:39:18', '2024-08-08 15:39:18');
INSERT INTO github.comments VALUES (2156807452, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156807452', 'Of course. What editor are you using? Can you share your config?', DEFAULT, '2024-06-09 21:59:27', '2024-06-09 21:59:27');
INSERT INTO github.comments VALUES (2156817348, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156817348', 'It looks like you meant to set the `auth_token` not the `auth_token_env_var_name` https://github.com/SilasMarvin/lsp-ai/wiki/Configuration#openai-compatible-apis', DEFAULT, '2024-06-09 22:27:01', '2024-06-09 22:27:01');
INSERT INTO github.comments VALUES (2157062831, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2157062831', 'Take a look at the prompting guide on the wiki: https://github.com/SilasMarvin/lsp-ai/wiki/Prompting

There is a lot more that goes into it.', DEFAULT, '2024-06-10 02:20:59', '2024-06-10 02:20:59');
INSERT INTO github.comments VALUES (2174206497, 2357003247, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2174206497', 'This is looking really good! The only major note is that I don''t think you actually want to support FIM. If you look at the mistral FIM, you see we supply both a prompt and suffix. The API for Mistral FIM specifically performs FIM. I think you want to support generateText and generateContent endpoints for the Gemini API.

Instead of do_fim you will probably want do_generate. You can see the Ollama implementation for an example: https://github.com/SilasMarvin/lsp-ai/blob/cd46ecf61adc272491c25c4faf4b5f9d92c3d4a6/src/transformer_backends/ollama.rs#L63

I feel like the Ollama implementation is probably a better example to go off of than the Mistral FIM one. Let me know if this doesn''t make sense and I am happy to clarify anything!', DEFAULT, '2024-06-17 18:54:11', '2024-06-17 18:54:11');
INSERT INTO github.comments VALUES (2406736479, 2563306159, 1893361, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2406736479', ':facepalm: fixed the early close of initializationOptions and is shows actions, thanks. It wont complete yet, will later debug what gpt4o is returning.', DEFAULT, '2024-10-11 07:38:27', '2024-10-11 07:38:27');
INSERT INTO github.comments VALUES (2162115425, 2345377798, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2162115425', 'I spent some time playing around with this. As you saw, I did see the completion requests being fulfilled by LSP-AI, but Zed doesn''t always show them. I think this might be a bug with Zed, I''m not sure. It would be really awesome to have a good integration with their editor. Maybe create an issue on their github? If you do, can you tag me in it, or send me the link, I would love to see what they say. 

Also, how do you find codegemma''s results? I was not able to get it to produce good outputs. I found llama3-8b with some decent prompting greatly outperforms it, but maybe I am prompting it incorrectly.', DEFAULT, '2024-06-12 05:07:45', '2024-06-12 05:07:45');
INSERT INTO github.comments VALUES (2161314990, 2343344751, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2161314990', '> resolved, [3a351ff](https://github.com/SilasMarvin/lsp-ai/pull/15/commits/3a351ff4a0ed846ceb4bc9d926ac422c8324a8a1)[237f4cf](https://github.com/SilasMarvin/lsp-ai/commit/237f4cf179b79f74b4ad2e169e8e4f855f96a6b4)

Awesome thank you. If you can implement the changes I suggested above I will merge it', DEFAULT, '2024-06-11 17:53:28', '2024-06-11 17:53:28');
INSERT INTO github.comments VALUES (2156438701, 2341830542, 68561, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156438701', 'Ah, you''re right about the max_tokens… I wasn''t sure as I followed the recommended config.
Could''ve tried though.

Anyway… it''s now at 4096 and this fixes the aforementioned issue.', DEFAULT, '2024-06-09 10:46:04', '2024-06-09 10:46:05');
INSERT INTO github.comments VALUES (2271639165, 2451215027, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2271639165', 'I''ll look into this thank you!', DEFAULT, '2024-08-06 16:02:43', '2024-08-06 16:02:43');
INSERT INTO github.comments VALUES (2158298884, 2341547917, 304428, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2158298884', 'I''ll be opening a draft PR in a bit. I would not recommend merging it until the default config is integrated in `nvim-lspconfig` (managing the LSP lifecycle by hand is annoying and exactly what `nvim-lspconfig` is here for) but at least it should give a place to point the more adventurous people that want to try it _right now_.

Edit: PR up, see #17 ', DEFAULT, '2024-06-10 13:05:05', '2024-06-10 13:46:42');
INSERT INTO github.comments VALUES (2156815990, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156815990', 'Can you close VS Code, go to your terminal in a directory you want to open with VS Code. Run `export LSP_AI_LOG=DEBUG` and then `code .`. 

This will Open VS Code with the environment variable `LSP_AI_LOG=DEBUG` which enables debugging for LSP-AI.', DEFAULT, '2024-06-09 22:22:38', '2024-06-09 22:22:38');
INSERT INTO github.comments VALUES (2157120924, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2157120924', 'That is strange it is not providing the complete code. I would test in their playground and see if it works there? Maybe they have some hidden API limits? I don''t think our request is malformed. 

Slow and steady wins the race!', DEFAULT, '2024-06-10 03:21:26', '2024-06-10 03:21:26');
INSERT INTO github.comments VALUES (2163771086, 2343344751, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2163771086', '> Is github not working as intended? i cannot see them on my side, nor my mailbox, i will make the change shown in your screenshot ![image](https://private-user-images.githubusercontent.com/43348732/339102890-04cd4287-1f31-4ac9-810b-d72067fe8345.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MTgyMjEzNzYsIm5iZiI6MTcxODIyMTA3NiwicGF0aCI6Ii80MzM0ODczMi8zMzkxMDI4OTAtMDRjZDQyODctMWYzMS00YWM5LTgxMGItZDcyMDY3ZmU4MzQ1LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNDA2MTIlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjQwNjEyVDE5Mzc1NlomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWJmOGVkMDQ0ZDJjNzMzNmE1MDM5YzU5NjE4ZTUwM2FlZmNiY2UzMzI2MTYzMTZiY2M5NTY0NDI4YTRjODJiNzkmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JmFjdG9yX2lkPTAma2V5X2lkPTAmcmVwb19pZD0wIn0.JmilMPS6t1nv0t2GHHYQpfgesudDw82Y439Zna9fK-8)
> 
> > I liked the defaults you had before.
> 
> i will revert them at next commit
> 
> `suggested by gpt-4o: try "submitreview"`

Oh super strange. I have attached the two other suggestions that I made. 

<img width="875" alt="Screenshot 2024-06-12 at 12 38 12 PM" src="https://github.com/SilasMarvin/lsp-ai/assets/19626586/6aed466d-90b1-4e44-8fed-1ea2c3ecd02b">

<img width="879" alt="Screenshot 2024-06-12 at 12 38 16 PM" src="https://github.com/SilasMarvin/lsp-ai/assets/19626586/432dd425-cfac-4d55-b740-67296e922914">', DEFAULT, '2024-06-12 19:40:13', '2024-06-12 19:40:13');
INSERT INTO github.comments VALUES (2184401788, 2368077996, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2184401788', 'Glad you like it! Thanks for flagging this. I''ll test this locally and get a fix for it in the next release!', DEFAULT, '2024-06-23 02:42:25', '2024-06-23 02:42:25');
INSERT INTO github.comments VALUES (2252224221, 2345377798, 71929976, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2252224221', '> I understand that lsp-ai has a different range of supported frameworks and is flexible in different ways, so it''s still exciting to see a proper Zed extension, 

Yes that''s why it''s cool and a alternative to copilot or codeium like tools

> but some users may find it is already not necessary for their use cases

Maybe but many peoples wants other tools instead of copilot ', DEFAULT, '2024-07-26 08:21:17', '2024-07-26 08:21:17');
INSERT INTO github.comments VALUES (2163838750, 2343344751, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2163838750', 'Looks pretty good! 

You should be able to change:
```
.post(self
                .configuration
                .generate_endpoint
                .as_ref()
                .unwrap_or(&"http://localhost:11434/api/generate".to_string())
```

To:
```
.post(self.configuration.generate_endpoint.as_deref().unwrap_or("http://localhost:11434/api/generate"))
```', DEFAULT, '2024-06-12 20:25:47', '2024-06-12 20:25:47');
INSERT INTO github.comments VALUES (2161616458, 2341830542, 68561, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2161616458', 'That did it, thanks!

I used this config to start out: https://github.com/SilasMarvin/lsp-ai/wiki/Configuration#chat-4 – it uses max_new_tokens.', DEFAULT, '2024-06-11 21:21:21', '2024-06-11 21:21:21');
INSERT INTO github.comments VALUES (2156806936, 2341547917, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156806936', '> For those of you who want a _very rough pow_ you can use this snippet. This will use ~ClosedAI~ OpenAI''s chat completion and can be called using `<leader>co`. This is pretty much just a work in progress, but maybe it will help someone :)
> 
> ```lua
> local lsp_ai_config = {
>   -- Uncomment if using nvim-cmp
>   -- capabilities = require(''cmp_nvim_lsp'').default_capabilities(),
>   cmd = { ''lsp-ai'' },
>   root_dir = vim.loop.cwd(),
>   init_options = {
>     memory = {
>       file_store = {}
>     },
>     models = {
>       model1 = {
>           type = "open_ai",
>           chat_endpoint = "https://api.openai.com/v1/chat/completions",
>           model = "gpt-4-1106-preview",
>           auth_token_env_var_name = "OPENAI_API_KEY",
>       }
>     },
>     completion = {
>       model = "model1",
>       parameters = {
>           max_context = 2048,
>           max_new_tokens = 128,
>             messages = {
>             {
>               role = "system",
>               content = "You are a chat completion system like GitHub Copilot. You will be given a context and a code snippet. You should generate a response that is a continuation of the context and code snippet."
>             },
>             {
>               role = "user",
>               content = "Context: {CONTEXT} - Code: {CODE}"
>             }
>           }
>       }
>     }
>   },
> }
> 
> vim.api.nvim_create_autocmd({"BufEnter", "BufWinEnter"}, {
>   callback = function() vim.lsp.start(lsp_ai_config) end,
> })
> 
> -- Register key shortcut
> vim.keymap.set(
>     "n", 
>     "<leader>co", 
>     function()
>         print("Loading completion...")
> 
>         local x = vim.lsp.util.make_position_params(0)
>         local y = vim.lsp.util.make_text_document_params(0)
> 
>         local combined = vim.tbl_extend("force", x, y)
> 
>         local result = vim.lsp.buf_request_sync(
>             0,
>             "textDocument/completion",
>             combined,
>             10000
>         )
> 
>         print(vim.inspect(result))
>     end,
>     {
>         noremap = true,
>     }
> )
> ```
> 
> I''d definitely wish a ghost-like text, just like [copilot.vim](https://github.com/github/copilot.vim) does it. I''m not too familiar with LSPs, but #5 could be related to this

Thank you for sharing this! To integrate fully with Neovim and provide good inline completion with ghost text I think we will need to write our own plugin. Write now it will pretty much mimic the functionality of copilot.vim but have more support for different backends for completion. This will change as we add new supported features to LSP-AI that we want Neovim to take advantage of like chatting with your code and semantic search over your code base.

If anyone sees this and is interested in writing a Neovim plugin, feel free to do it! I''m happy to help however I can. Our VS Code plugin is a really good place to start for the kind of functionality it should provide: https://github.com/SilasMarvin/lsp-ai/blob/main/editors/vscode/src/index.ts', DEFAULT, '2024-06-09 21:57:48', '2024-06-09 21:57:48');
INSERT INTO github.comments VALUES (2156813845, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156813845', 'Got it, that looks like its working then. I think it might be a setting in your VS Code configuration. 

Can you make sure your `Editor > Inline Suggest: Enabled` is checked. 

Also make sure the value of `Editor > Quick Suggestions` `other` is set to inline.', DEFAULT, '2024-06-09 22:16:12', '2024-06-09 22:16:12');
INSERT INTO github.comments VALUES (2156818603, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156818603', 'Glad I could help! Just a note, the responses from OpenAI right now are not going to be very good with the filler messages. You will want to replace those. I have a section on the wiki for prompting that may be worth looking at', DEFAULT, '2024-06-09 22:31:07', '2024-06-09 22:31:07');
INSERT INTO github.comments VALUES (2157085328, 2342506239, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2157085328', 'Sure I think the implementation could work like that. My main idea though is that by labeling a bunch of files, collectively they can be joined to deliver much more specific/fine tuned results. 

Imagine the README labels the app as a CSM, its framework & dependencies and lsp-ai picks it up. We could then, like you said earlier, use RAG for better results on subsequent methods/classes/files it generates code for.

I would hope provided that a dev could then create a new file named `User.model.js`  and then the model would be able to, without prompt, generate a basic dependency definition for a user model(sequelize vs mongoose for example).

I''m looking into the prompting dance per your suggestion and doing research on all the other plumbing like vscode-languageclient so gimme some time =).', DEFAULT, '2024-06-10 02:37:02', '2024-06-10 02:42:16');
INSERT INTO github.comments VALUES (2406027117, 2563306159, 147047, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2406027117', 'Not 100% on this, but I think your json config is malformed:

https://github.com/SilasMarvin/lsp-ai/wiki/Configuration#custom-actions

The instructions are a bit unclear here, but I think the "initializeOptions" is inferred to be the entire json object', DEFAULT, '2024-10-10 20:59:00', '2024-10-10 20:59:00');
INSERT INTO github.comments VALUES (2161230179, 2345377798, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2161230179', 'Got it, thank you! I will look at this tonight.', DEFAULT, '2024-06-11 17:01:31', '2024-06-11 17:01:31');
INSERT INTO github.comments VALUES (2159714632, 2343344751, 43348732, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2159714632', '> This is really awesome thank you!
> 
> What do you think about explicitly adding a `chat_endpoint` and a `generate_endpoint` in the config options for Ollama instead of just `endpoint`? We do this for every other API backend. If a user uses Ollama remotely are we sure they will have chat at `/api/chat` and generate at `/api/generate`?

I think Adding separate chat_endpoint and generate_endpoint for Ollama isn''t necessary for now because once Ollama is running, it provides both chat and generate functionalities at /api/chat and /api/generate. But it is Fine to have 2 config options, providing better consistency across providers', DEFAULT, '2024-06-11 03:32:41', '2024-06-11 03:58:59');
INSERT INTO github.comments VALUES (2163841445, 2343344751, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2163841445', 'No worries! Awesome work, I''m going to test locally and merge it tonight!

Thanks for contributing!', DEFAULT, '2024-06-12 20:27:27', '2024-06-12 20:27:40');
INSERT INTO github.comments VALUES (2271633859, 2451215027, 32483567, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2271633859', '```
stack backtrace:
   0:     0x55c5f6c09e70 - ::fmt::h708de712029cb15f
   1:     0x55c5f6c4210b - core::fmt::write::h1c4353b51cccb3ea
   2:     0x55c5f6c2daf9 - std::io::Write::write_fmt::hcc8780934fe9031f
   3:     0x55c5f6c09c2e - std::sys_common::backtrace::print::hc2ff0f9eb8716380
   4:     0x55c5f6c1b6fa - std::panicking::default_hook::{{closure}}::h2608c1692f29128d
   5:     0x55c5f6c1b3e9 - std::panicking::default_hook::hb19e8d974208f968
   6:     0x55c5f6c1bb89 - std::panicking::rust_panic_with_hook::hed421d2c4566430f
   7:     0x55c5f6c0a244 - std::panicking::begin_panic_handler::{{closure}}::h02e89190bc8024e5
   8:     0x55c5f6c0a089 - std::sys_common::backtrace::__rust_end_short_backtrace::h2501369998c7775f
   9:     0x55c5f6c1b887 - rust_begin_unwind
  10:     0x55c5f5f0c573 - core::panicking::panic_fmt::h02e8faf7efcfa656
  11:     0x55c5f6c47a1a - core::str::slice_error_fail_rt::hcbad84ada2c98528
  12:     0x55c5f5f0c99a - core::str::slice_error_fail::hca52f8f50664a623
  13:     0x55c5f60a2b37 - lsp_ai::transformer_worker::post_process_start::he88dabbac10745d4
  14:     0x55c5f60a3093 - lsp_ai::transformer_worker::post_process_response::h4dc4c8396ad52198
  15:     0x55c5f5fed404 - lsp_ai::transformer_worker::generate_response::{{closure}}::h2d3fb4f29c6a758a
  16:     0x55c5f5fe4dd3 -  as core::future::future::Future>::poll::hfc6661050e41f5e4
  17:     0x55c5f5f68e2c - lsp_ai::transformer_worker::do_run::{{closure}}::{{closure}}::ha38286bd1b1e962f
  18:     0x55c5f5f59a6e - tokio::runtime::task::core::Core::poll::hfb97ec53e6049e76
  19:     0x55c5f6112b71 - tokio::runtime::task::raw::poll::h713c49f34236467f
  20:     0x55c5f69ff5fb - tokio::runtime::scheduler::multi_thread::worker::Context::run_task::h4a499f1ef10f0a98
  21:     0x55c5f69fea75 - tokio::runtime::scheduler::multi_thread::worker::Context::run::hcca3183b6b1081e6
  22:     0x55c5f6a07407 - tokio::runtime::context::scoped::Scoped::set::h535d44a70c43585e
  23:     0x55c5f6a0fd7c - tokio::runtime::context::runtime::enter_runtime::hf0ccb4529c2444ae
  24:     0x55c5f69fe126 - tokio::runtime::scheduler::multi_thread::worker::run::h39f771055e77b9ad
  25:     0x55c5f6a04503 -  as core::future::future::Future>::poll::haa54e967bf82f185
  26:     0x55c5f6a09848 - tokio::runtime::task::core::Core::poll::ha5000f2d4e22d52b
  27:     0x55c5f6a03482 - tokio::runtime::task::harness::Harness::poll::h060d963f572f5c2e
  28:     0x55c5f6a1377d - tokio::runtime::blocking::pool::Inner::run::h7548a72e2c9cf6b8
  29:     0x55c5f6a14fcc - std::sys_common::backtrace::__rust_begin_short_backtrace::h3fc14da953569c77
  30:     0x55c5f6a0e5d2 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hc734c5c0502affb6
  31:     0x55c5f6c227ab - std::sys::pal::unix::thread::Thread::new::thread_start::hd8a8926ef174e878
  32:     0x7fed77a92ba2 - start_thread
  33:     0x7fed77b1400c - clone3
  34:                0x0 - 
```', DEFAULT, '2024-08-06 16:00:00', '2024-08-06 16:00:00');
INSERT INTO github.comments VALUES (2158283964, 2341547917, 50424412, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2158283964', '@Robzz we''d love to see how you did that! :)', DEFAULT, '2024-06-10 12:59:06', '2024-06-10 12:59:06');
INSERT INTO github.comments VALUES (2156815338, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156815338', '> I''ve been wanting to find a os project to work on for some time now but hadn''t had any luck. Felt like all other apps have been handled. I hope to be able to contribute to this one since it''s so early and I''m sure you could use help and have other ideas. I haven''t spun up an electron app is years though, lol. Also I know little about ML and even less about Rust so yeah... appreciate your work, looking forward to this becoming a "have to have" app on VSCode.

I am absolutely looking for help. I have a bunch of ideas and would love to talk about what areas of development interest you! There is a ton of really exciting things to do here. 

If you are interested in the VS Code side of things, there is a lot still to do with the plugin we have written: https://github.com/SilasMarvin/lsp-ai/blob/main/editors/vscode/src/index.ts Copilot''s plugin has a lot more built out features and while I don''t necessarily think we need to match its features, I think there should be some discussion around what features we do want.

If you want to get more into the Rust / language server side of things, there is also a bunch to do there. I want to explore agent based systems and have some discussion around what integrating something like that directly into the backend would look like. I''m currently working on directory crawling and RAG.', DEFAULT, '2024-06-09 22:20:23', '2024-06-09 22:23:44');
INSERT INTO github.comments VALUES (2156835483, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156835483', '> > > 
> > 
> > 
> > If you are interested in the VS Code side of things, there is a lot still to do with the plugin we have written: https://github.com/SilasMarvin/lsp-ai/blob/main/editors/vscode/src/index.ts Copilot''s plugin has a lot more built out features and while I don''t necessarily think we need to match its features, I think there should be some discussion around what features we do want.
> > If you want to get more into the Rust / language server side of things, there is also a bunch to do there. I want to explore agent based systems and have some discussion around what integrating something like that directly into the backend would look like. I''m currently working on directory crawling and RAG.
> 
> Very interested in getting making something for progeny. Lemme know what your ideas are and I''ll have a look at it? It''s your baby so I feel you should have that kinda executive product power right now.

I appreciate your thoughts. If you want to participate, I want you to build out something you are excited about too! Check out the following capabilities of Copilot: https://code.visualstudio.com/docs/copilot/overview Let me know what you think about these capabilities and if we should try and match them.

I''m going to mark this as closed but we can continue talking here, or you can create another issue for discussing a good feature to add.', DEFAULT, '2024-06-09 23:02:33', '2024-06-09 23:02:33');
INSERT INTO github.comments VALUES (2159418063, 2341830542, 68561, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2159418063', '```toml
[language-server.lsp-ai]
command = "lsp-ai"
args = ["--stdio"]

[language-server.lsp-ai.config.memory]
file_store = {}

[language-server.lsp-ai.config.models.model1]
type = "anthropic"
chat_endpoint = "https://api.anthropic.com/v1/messages"
model = "claude-3-sonnet-20240229"
auth_token_env_var_name = "ANTHROPIC_API_KEY"

[language-server.lsp-ai.config.completion]
model = "model1"

[language-server.lsp-ai.config.completion.parameters]
max_context = 2048
max_new_tokens = 128
system = "Instructions:\n- You are an AI programming assistant.\n- Given a piece of code with the cursor location marked by \"<CURSOR>\", replace \"<CURSOR>\" with the correct code or comment.\n- First, think step-by-step.\n- Describe your plan for what to build in pseudocode, written out in great detail.\n- Then output the code replacing the \"<CURSOR>\"\n- Ensure that your completion fits within the language context of the provided code snippet (e.g., Python, JavaScript, Rust).\n\nRules:\n- Only respond with code or comments.\n- Only replace \"<CURSOR>\"; do not include any previously written code.\n- Never include \"<CURSOR>\" in your response\n- If the cursor is within a comment, complete the comment meaningfully.\n- Handle ambiguous cases by providing the most contextually appropriate completion.\n- Be consistent with your responses."


[[language-server.lsp-ai.config.completion.parameters.messages]]
role = "user"
content = "def greet(name):\n    print(f\"Hello, {<CURSOR>}\")"

[[language-server.lsp-ai.config.completion.parameters.messages]]
role = "assistant"
content = "name"

[[language-server.lsp-ai.config.completion.parameters.messages]]
role = "user"
content = "function sum(a, b) {\n    return a + <CURSOR>;\n}"

[[language-server.lsp-ai.config.completion.parameters.messages]]
role = "assistant"
content = "b"

[[language-server.lsp-ai.config.completion.parameters.messages]]
role = "user"
content = "fn multiply(a: i32, b: i32) -> i32 {\n    a * <CURSOR>\n}"

[[language-server.lsp-ai.config.completion.parameters.messages]]
role = "assistant"
content = "b"

[[language-server.lsp-ai.config.completion.parameters.messages]]
role = "user"
content = "# <CURSOR>\ndef add(a, b):\n    return a + b"

[[language-server.lsp-ai.config.completion.parameters.messages]]
role = "assistant"
content = "Adds two numbers"

[[language-server.lsp-ai.config.completion.parameters.messages]]
role = "user"
content = "# This function checks if a number is even\n<CURSOR>"

[[language-server.lsp-ai.config.completion.parameters.messages]]
role = "assistant"
content = "def is_even(n):\n    return n % 2 == 0"

[[language-server.lsp-ai.config.completion.parameters.messages]]
role = "user"
content = "{CODE}"

[language-server.tailwindcss-vue]
command = "tailwindcss-language-server"
args = ["--stdio"]
language-id = "vue"
timeout = 2

[language-server.typescript-language-server.config]
plugins = [
  { name = "@vue/typescript-plugin", location = "node_modules/@vue/language-server", languages = ["vue"] }
]

[language-server.vue-language-server]
command = "vue-language-server"
args = ["--stdio"]
config = { vue = { hybridMode = true }, typescript = { tsdk = "node_modules/typescript/lib" } }

[[language]]
name = "vue"
auto-format = true
formatter = { command = "prettier", args = ["--parser", "vue"] }
language-servers = ["typescript-language-server", "lsp-ai", "vue-language-server", "tailwindcss-vue"]
indent = { tab-width = 2, unit = "  " }

[[language]]
name = "typescript"
auto-format = true
formatter = { command = "prettier", args = ["--parser", "typescript"] }
language-servers = ["typescript-language-server"]

[[language]]
name = "elixir"
auto-format = true
language-servers = ["elixir-ls"]
```', DEFAULT, '2024-06-10 22:30:40', '2024-06-10 22:30:40');
INSERT INTO github.comments VALUES (2156030972, 2341547917, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156030972', 'Hey @Nold360 . We were just having this discussion on Reddit yesterday and will hopefully have some example configurations in the repository and make a pr into `nvim-lspconfig` soon. 

Here is an answer provided by Microbzz on reddit:
```
local lsp_ai_config = {
  -- Uncomment if using nvim-cmp
  -- capabilities = require(''cmp_nvim_lsp'').default_capabilities(),
  cmd = { ''lsp-ai'' },
  root_dir = vim.loop.cwd(),
  init_options = {
    memory = {
      file_store = {}
    },
    models = {
      model1 = {
        type = "llama_cpp",
        repository = "mmnga/codegemma-1.1-2b-gguf",
        name = "codegemma-1.1-2b-Q8_0.gguf",
        n_ctx = 2048,
        n_gpu_layers = 999
      }
    },
    completion = {
      model = "model1",
      parameters = {
        fim = {
          start = "<|fim_prefix|>",
          middle = "<|fim_suffix|>",
          ["end"] = "<|fim_middle|>"
        },
        max_context = 2000,
        max_new_tokens = 32
      }
    }
  },
}

vim.api.nvim_create_autocmd({"BufEnter", "BufWinEnter"}, {
  callback = function() vim.lsp.start(lsp_ai_config) end,
})
```

You can swap out the  value of `init_options` with whatever configuration you prefer. See the [configuration section](https://github.com/SilasMarvin/lsp-ai/wiki/Configuration) of the wiki for more info. 

There is still an open discussion around getting ghost text working and potentially shipping our own neovim plugin for automatic inline completion. ', DEFAULT, '2024-06-08 13:06:08', '2024-06-08 13:06:08');
INSERT INTO github.comments VALUES (2156811511, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156811511', 'Thank you! In your VS Code terminal in the Output tab, can you change the dropdown from Window to `lsp-ai` and send me what it prints out? 

Are you on mac, did you install with `llama_cpp` and `metal` support?', DEFAULT, '2024-06-09 22:09:38', '2024-06-09 22:09:38');
INSERT INTO github.comments VALUES (2156817616, 2342506239, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156817616', '> > 
> If you are interested in the VS Code side of things, there is a lot still to do with the plugin we have written: https://github.com/SilasMarvin/lsp-ai/blob/main/editors/vscode/src/index.ts Copilot''s plugin has a lot more built out features and while I don''t necessarily think we need to match its features, I think there should be some discussion around what features we do want.
> 
> If you want to get more into the Rust / language server side of things, there is also a bunch to do there. I want to explore agent based systems and have some discussion around what integrating something like that directly into the backend would look like. I''m currently working on directory crawling and RAG.

Very interested in building something for progeny.  Lemme know what your ideas are and I''ll have a look at it? It''s your baby so I feel you should have that kinda executive product power right now.
', DEFAULT, '2024-06-09 22:27:59', '2024-06-10 02:27:47');
INSERT INTO github.comments VALUES (2157063400, 2342506239, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2157063400', '> and feel free to make a pull request anytime!

I''m looking forward to it. I think a killer feature which is missing in these types of tools(in my limited experimentation) is that we should be able to tag/label the file. The label/tag is then joined with the inline comment/prompts. 

I expect models could produce better results with this additional feature/dimension. Like add a comment on line 1 and that''s always sent with the subsequent prompt requests.

', DEFAULT, '2024-06-10 02:21:44', '2024-06-10 02:22:44');
INSERT INTO github.comments VALUES (2156805129, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156805129', 'Thanks for checking the project out. 

You should never have to start `lsp-ai` yourself. You should configure your text editor to do it for you. If you are using VS Code, we have an official VS Code plugin. If you are using helix or neovim, we have some example configurations available for those editors. If you are using a different editor, you will have to checkout your editors documentation around adding a new language server.', DEFAULT, '2024-06-09 21:51:29', '2024-06-09 21:51:29');
INSERT INTO github.comments VALUES (2156769076, 2341547917, 50424412, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156769076', 'For those of you who want a _very rough pow_ you can use this snippet. This will use ~ClosedAI~ OpenAI''s chat completion and can be called using `<leader>co`. This is pretty much just a work in progress, but maybe it will help someone :)

```lua
local lsp_ai_config = {
  -- Uncomment if using nvim-cmp
  -- capabilities = require(''cmp_nvim_lsp'').default_capabilities(),
  cmd = { ''lsp-ai'' },
  root_dir = vim.loop.cwd(),
  init_options = {
    memory = {
      file_store = {}
    },
    models = {
      model1 = {
          type = "open_ai",
          chat_endpoint = "https://api.openai.com/v1/chat/completions",
          model = "gpt-4-1106-preview",
          auth_token_env_var_name = "OPENAI_API_KEY",
      }
    },
    completion = {
      model = "model1",
      parameters = {
          max_context = 2048,
          max_new_tokens = 128,
            messages = {
            {
              role = "system",
              content = "You are a chat completion system like GitHub Copilot. You will be given a context and a code snippet. You should generate a response that is a continuation of the context and code snippet."
            },
            {
              role = "user",
              content = "Context: {CONTEXT} - Code: {CODE}"
            }
          }
      }
    }
  },
}

vim.api.nvim_create_autocmd({"BufEnter", "BufWinEnter"}, {
  callback = function() vim.lsp.start(lsp_ai_config) end,
})

-- Register key shortcut
vim.keymap.set(
    "n", 
    "<leader>co", 
    function()
        print("Loading completion...")

        local x = vim.lsp.util.make_position_params(0)
        local y = vim.lsp.util.make_text_document_params(0)

        local combined = vim.tbl_extend("force", x, y)

        local result = vim.lsp.buf_request_sync(
            0,
            "textDocument/completion",
            combined,
            10000
        )

        print(vim.inspect(result))
    end,
    {
        noremap = true,
    }
)
```

I''d definitely wish a ghost-like text, just like [copilot.vim](https://github.com/github/copilot.vim) does it. I''m not too familiar with LSPs, but #5 could be related to this', DEFAULT, '2024-06-09 20:12:53', '2024-06-09 20:12:53');
INSERT INTO github.comments VALUES (2156812516, 2342506239, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156812516', 'Yes I did install with metal per suggestions in the README. Sorry I was trying to illustrate shortcut press. Here''s the terminal output.

```sh
llama_kv_cache_init:      Metal KV buffer size =   640.00 MiB
llama_new_context_with_model: KV self size  =  640.00 MiB, K (f16):  320.00 MiB, V (f16):  320.00 MiB
llama_new_context_with_model:        CPU  output buffer size =     0.19 MiB
llama_new_context_with_model:      Metal compute buffer size =   152.00 MiB
llama_new_context_with_model:        CPU compute buffer size =     9.01 MiB
llama_new_context_with_model: graph nodes  = 1095
llama_new_context_with_model: graph splits = 2
llama_new_context_with_model: n_ctx      = 2048
llama_new_context_with_model: n_batch    = 2048
llama_new_context_with_model: n_ubatch   = 512
llama_new_context_with_model: flash_attn = 0
llama_new_context_with_model: freq_base  = 10000.0
llama_new_context_with_model: freq_scale = 1
llama_kv_cache_init:      Metal KV buffer size =   640.00 MiB
llama_new_context_with_model: KV self size  =  640.00 MiB, K (f16):  320.00 MiB, V (f16):  320.00 MiB
llama_new_context_with_model:        CPU  output buffer size =     0.19 MiB
llama_new_context_with_model:      Metal compute buffer size =   152.00 MiB
llama_new_context_with_model:        CPU compute buffer size =     9.01 MiB
llama_new_context_with_model: graph nodes  = 1095
llama_new_context_with_model: graph splits = 2
```', DEFAULT, '2024-06-09 22:12:14', '2024-06-09 22:12:14');
INSERT INTO github.comments VALUES (2156816655, 2342506239, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156816655', 'Regardless I have the OPEN AI one set actually. 
<img width="2056" alt="Screenshot 2024-06-09 at 6 22 18 PM" src="https://github.com/SilasMarvin/lsp-ai/assets/16382165/e7f80335-0218-44c2-ba14-a5636ea6823d">


```sh
ERROR dispatch_request{request=Generation(GenerationRequest { id: RequestId(I32(1)), params: GenerationParams { text_document_position: TextDocumentPositionParams { text_document: TextDocumentIdentifier { uri: Url { scheme: "file", cannot_be_a_base: false, username: "", password: None, host: None, port: None, path: "/Users/future/Documents/Work/_Main/.Projects/experiment/main.py", query: None, fragment: None } }, position: Position { line: 4, character: 0 } }, model: "model1", parameters: Object {"max_context": Number(1024), "max_tokens": Number(128), "messages": Array [Object {"content": String("SOME CUSTOM SYSTEM MESSAGE"), "role": String("system")}, Object {"content": String("SOME CUSTOM USER MESSAGE WITH THE {CODE}"), "role": String("user")}]} } })}: lsp_ai::transformer_worker: generating response: environment variable not found
DEBUG lsp_server::msg: > {"jsonrpc":"2.0","id":1,"error":{"code":-32603,"message":"environment variable not found"}}    
```', DEFAULT, '2024-06-09 22:24:43', '2024-06-09 22:25:42');
INSERT INTO github.comments VALUES (2157061375, 2342506239, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2157061375', 'Isn''t it on line 85, the comment? 

We prompt the AI in comments is my understanding . However it''s been a while since I used one so correct me if I''m wrong.', DEFAULT, '2024-06-10 02:19:07', '2024-06-10 02:19:07');
INSERT INTO github.comments VALUES (2157115672, 2342506239, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2157115672', 'Here''s my generation prompt config. 

```sh
"lsp-ai.generationConfiguration": {
    "model": "model1",
    "parameters": {
      "max_tokens": 2000,
      "max_context": 3000,
      "messages": [
        {
          "role": "system",
          "content": "You are a Generative AI fine tuned for software development. You''ve mastered mobile, web, and AI/ML concepts, languages, frameworks, & conventions. You help the user complete the code you''re provided."
        },
        {
          "role": "user",
          "content": "{CODE}"
        }
      ]
    }
  },
```

Take away for me is that despite defining a much higher token allocation it failed to provide the "complete" code. Not sure what the issue is, my prompt in the config or the editor...? 

Slow and steady, lol =)', DEFAULT, '2024-06-10 03:16:09', '2024-06-10 03:23:29');
INSERT INTO github.comments VALUES (2159257983, 2341547917, 71392160, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2159257983', 'For "ghost text" I am working on features for [supermaven-nvim](https://github.com/supermaven-inc/supermaven-nvim) and we use something like this:

- https://github.com/supermaven-inc/supermaven-nvim/blob/ef3bd1a6b6f722857f2f88d929dd4ac875655611/lua/supermaven-nvim/completion_preview.lua#L59-L88

I didn''t implement this feature so I am not gonna say I completely get it but when creating the `autocmd` adding a namespace like `lsp-ai` for example and when getting the result add it to a `api-extended-marks`, RTFM for more information on that.

```lua
local augroup = vim.api.nvim_create_augroup("lsp-ai", { clear = true })
local ns_id = vim.api.nvim_create_namespace("lsp-ai")
local opts = {
  id = 1,
  hl_mode = "combine",
}

vim.api.nvim_create_autocmd({"BufEnter", "BufWinEnter"}, {
  group = augroup,
  callback = function() vim.lsp.start(lsp_ai_config) end,
})

-- Register key shortcut
vim.keymap.set(
    "n", 
    "<leader>co", 
    function()
        print("Loading completion...")

        local x = vim.lsp.util.make_position_params(0)
        local y = vim.lsp.util.make_text_document_params(0)

        local combined = vim.tbl_extend("force", x, y)

        local result = vim.lsp.buf_request_sync(
            0,
            "textDocument/completion",
            combined,
            10000
        )

        print(vim.inspect(result))
        vim.api.nvim_buf_set_extmark(0, ns_id, vim.fn.line(".") - 1, vim.fn.col(".") - 1, opts)
    end,
    {
        noremap = true,
    }
)
```

> [!WARNING]
>
> Not sure this works as expected for ghost text or how the `result` comes in. So you may have to modified and play around a bit.', DEFAULT, '2024-06-10 20:48:43', '2024-06-10 20:48:43');
INSERT INTO github.comments VALUES (2157048595, 2342506239, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2157048595', '<img width="2056" alt="Screenshot 2024-06-09 at 9 53 07 PM" src="https://github.com/SilasMarvin/lsp-ai/assets/16382165/dabaadde-f0c3-4083-be1c-2e8d2b5b9090">
<img width="2056" alt="Screenshot 2024-06-09 at 9 53 09 PM" src="https://github.com/SilasMarvin/lsp-ai/assets/16382165/2b94df4f-3507-4fea-88f8-b4fa7ea668c0">

I really am interested. I even tried to figure out how to fork the wiki and update it earlier but went in circles and gave up. lol.

I''ve noticed the results are what I expected, far from perfect. For example here the code blocks aren''t complete. 

I also noticed when languages are "changed" the extension doesn''t "catch up" quickly enough. For example imagine I''m building an app in Flutter/Dart but I have Python scripts to do certain things. If I switch from Python to Dart it still generates Python despite being in a `.dart` file.

Anyway not trying to nitpick just saying. In any case I ended up poking around and saw the extension isn''t even using Electron as I initially thought but some other packages so I need to spend some time getting familiar with those. Concerning CoPilot extension, undoubtedly, I''d love to be able to add those so gimme some time to do some research on these libs.

', DEFAULT, '2024-06-10 02:03:23', '2024-06-10 02:03:23');
INSERT INTO github.comments VALUES (2157111687, 2342506239, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2157111687', '<img width="2056" alt="Screenshot 2024-06-09 at 10 55 59 PM" src="https://github.com/SilasMarvin/lsp-ai/assets/16382165/a89ffc3f-d252-4532-b354-d31f86299273">

I read the prompts docs you provided but still don''t quite get it. Those looked like generation data to me, multiply & is_event for example. 

The prompts should be similar to what I''ve done on line 16 I thought? Tell the AI what I want, fix the error, and it picks up the context by itself. 

<img width="2056" alt="Screenshot 2024-06-09 at 11 06 30 PM" src="https://github.com/SilasMarvin/lsp-ai/assets/16382165/8abad163-dd84-4b94-a937-e0c2987de581">

Here''s another example of what I **think** the expected behavior should be, that the AI can pickup comments and generate from there. Ideally it''d then stop doing what it''s does here, suggesting the same thing code despite other "prompts" not being delivered on.

Me too. Gimme some time to catch up on what you''ve already done & then I''ll try to figure out how to "switch" the prompt/context. Unsure of what else the explanation is for this time of generation is currently...', DEFAULT, '2024-06-10 03:10:50', '2024-06-10 03:30:23');
INSERT INTO github.comments VALUES (2158816103, 2341547917, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2158816103', '> > I think you are probably right here. I do think they shouldn''t be defaults on the server, they should be defaults for the config / plugin to send to the server. I don''t want to have defaults for the server that might expose users codebases to third parties.
> 
> Agree''d. I think ideally the LSP part and LSP/IDE glue should have sensible defaults, but the backend/model config should probably be left for the user to pick. So as far as I can tell, that leaves only the `memory` key, which accepts only an empty file store for now ? In this case aside from registering the `textDocument/generation` action with nvim, there wouldn''t be that much to do.

Yes we can make the `memory` key have a default but I agree the model config should be controlled by the user. I just checked out your fork of nvim-lspconfig. Thank you for making that. I think for the `init_options` if we want to provide OpenAI gpt-4o as the default model we can use the example OpenAI chat config I have in the configuration section of the wiki: https://github.com/SilasMarvin/lsp-ai/wiki/Configuration#chat-3

Or maybe I am misunderstanding what is standard for nvim-lspconfig and its ok to provide a default that doesn''t full work without the user providing more parameters?', DEFAULT, '2024-06-10 16:25:11', '2024-06-10 16:29:45');
INSERT INTO github.comments VALUES (2164237561, 2341547917, 304428, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2164237561', '> Would you want to head up our neovim plugin? I''m thinking for now we just fork llm-ls, edit the configuration options to match the options I have for our VS Code plugin, and then just have it perform inline completion with ghost text.

I''m not sure how long term I can commit to it, but sure I''m happy to at least help it take it off the ground.

> I asked on the Matrix, they recommended discussing it on their GitHub. I suggest we create a PR that requires the user to provide defaults and have a discussion about it in that PR.

Alright I''ll send them the PR hopefully tomorrow to get the discussion going.', DEFAULT, '2024-06-13 02:32:24', '2024-06-13 02:32:24');
INSERT INTO github.comments VALUES (2169036263, 2341547917, 71392160, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2169036263', '@SuperBo for the `neorocks` I have something done in <https://github.com/AlejandroSuero/freeze-code.nvim>

Cherry pick what you want, is free to use.

I haven''t got to try `busted + nlua` yet, usually I stick to testing with neovim since it how my plugins interact most of the time. Any good places to take a look on how to start with `busted + nlua`?', DEFAULT, '2024-06-15 01:55:55', '2024-06-15 01:55:55');
INSERT INTO github.comments VALUES (2172471824, 2341547917, 994357, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2172471824', '@SuperBo oh, I was just looking at a README.md file when trying this out 😆 

If you like, we can continue this discussion in [my dotfiles/nvim PR](https://github.com/fredrikaverpil/dotfiles/pull/155), as I''m facing errors when opening a .go file inside [this repo](https://github.com/fredrikaverpil/neotest-golang).

```
   Warn  09:10:41 notify.warn Client lsp_ai quit with exit code 1 and signal 0. Check log for errors: /Users/fredrik/.local/state/fredrik/lsp.log
   Error  09:10:41 msg_show.lua_error Error executing vim.schedule lua callback: ....10.0/share/nvim/runtime/lua/vim/lsp/semantic_tokens.lua:185: Invalid buffer id: 18
stack traceback:
	[C]: in function ''nvim_buf_attach''
	....10.0/share/nvim/runtime/lua/vim/lsp/semantic_tokens.lua:185: in function ''new''
	....10.0/share/nvim/runtime/lua/vim/lsp/semantic_tokens.lua:612: in function ''start''
	.../neovim/0.10.0/share/nvim/runtime/lua/vim/lsp/client.lua:959: in function <.../neovim/0.10.0/share/nvim/runtime/lua/vim/lsp/client.lua:957>
   Error  09:10:42 msg_show.emsg LSP[gopls]: Error SERVER_REQUEST_HANDLER_ERROR: ".../Cellar/neovim/0.10.0/share/nvim/runtime/lua/vim/lsp.lua:310: Invalid buffer id: 18"
   Error  09:10:42 msg_show.lua_error Error executing vim.schedule lua callback: ....10.0/share/nvim/runtime/lua/vim/lsp/_changetracking.lua:154: Invalid buffer id: 19
stack traceback:
	[C]: in function ''nvim_buf_get_name''
	....10.0/share/nvim/runtime/lua/vim/lsp/_changetracking.lua:154: in function ''init''
	.../neovim/0.10.0/share/nvim/runtime/lua/vim/lsp/client.lua:907: in function ''_text_document_did_open_handler''
	.../neovim/0.10.0/share/nvim/runtime/lua/vim/lsp/client.lua:942: in function ''_on_attach''
	.../neovim/0.10.0/share/nvim/runtime/lua/vim/lsp/client.lua:615: in function ''''
	vim/_editor.lua: in function <vim/_editor.lua:0>
```', DEFAULT, '2024-06-17 07:14:58', '2024-06-17 12:49:43');
INSERT INTO github.comments VALUES (2157056797, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2157056797', 'What prompt are you using? The prompt will have a massive impact on how well the LLM performs. 

The Python and Dart mixup is interesting. We don''t actually specify the language the LLM is supposed to complete, we just pass it the code / comments around the cursor (this also depends on which prompt system you are using), so if you have Python or Dart code around the cursor it should complete correctly, but not always as LLMs are not deterministic and do have some strange quirks. 

That sounds great! Let me know what you think, and feel free to make a pull request anytime!', DEFAULT, '2024-06-10 02:14:33', '2024-06-10 02:14:33');
INSERT INTO github.comments VALUES (2157113877, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2157113877', 'The prompts that I have written for OpenAI are designed to make it perform code completion. You can alter the prompts however you see best! If you want to tell it what you want, and have it generate the code that way, you totally can. Just later the messages you send.

I recommend testing your prompts in the OpenAI playground. I find that to make prompt building easier than constant trial and error with editing the config.', DEFAULT, '2024-06-10 03:13:36', '2024-06-10 03:14:58');
INSERT INTO github.comments VALUES (2158920146, 2341547917, 304428, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2158920146', '> Or maybe I am misunderstanding what is standard for nvim-lspconfig and its ok to provide a default that doesn''t full work without the user providing more parameters?

This I really don''t know, maybe there are similar cases among the supported LSP servers in nvim-lspconfig, I haven''t seen any but I only glanced over it and it''s a *long* list. I haven''t seen any AI LSP servers in there either since HuggingFace have gone the way of writing their own plugin instead for `llm-ls` so there''s really no one to copy from. Asking on the nvim Matrix server is probably the easiest way to know for sure.', DEFAULT, '2024-06-10 17:26:23', '2024-06-10 17:26:35');
INSERT INTO github.comments VALUES (2164577222, 2341547917, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2164577222', '> > Would you want to head up our neovim plugin? I''m thinking for now we just fork llm-ls, edit the configuration options to match the options I have for our VS Code plugin, and then just have it perform inline completion with ghost text.
> 
> I''m not sure how long term I can commit to it, but sure I''m happy to at least help it take it off the ground.
> 
> > I asked on the Matrix, they recommended discussing it on their GitHub. I suggest we create a PR that requires the user to provide defaults and have a discussion about it in that PR.
> 
> Alright I''ll send them the PR hopefully tomorrow to get the discussion going.

This is awesome thank you!', DEFAULT, '2024-06-13 06:22:43', '2024-06-13 06:22:43');
INSERT INTO github.comments VALUES (2169066801, 2341547917, 2666479, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2169066801', '@AlejandroSuero, you can see the sample setup here https://github.com/SuperBo/fugit2.nvim.

I decided to fork from https://github.com/huggingface/llm.nvim. I saw some of your pullrequest (https://github.com/huggingface/llm.nvim/pull/98, https://github.com/huggingface/llm.nvim/pull/97) there. Could I merge it into my fork :D?', DEFAULT, '2024-06-15 02:45:29', '2024-06-15 02:45:29');
INSERT INTO github.comments VALUES (2195084454, 2341547917, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2195084454', '> @fredrikaverpil, what language are you testing (python, go, ...). I''ve just hard code these support file types: `{ "go", "java", "python", "rust" }`. Will make this configurable later.
> 
> Do you have lsp-ai compiled with llama_cpp? You can test with openai if you have OPENAI key.
> 
> You don''t need to add config to `dependencies`.

Following up on this thread. Been talking in the Discord a little bit about our Neovim integration and would love to get you in there @SuperBo Link is in the README', DEFAULT, '2024-06-27 15:54:33', '2024-06-27 15:54:33');
INSERT INTO github.comments VALUES (2200475111, 2343972535, 304428, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2200475111', 'Just a heads up that I haven''t forgotten about this, just got a bit busy lately plus my ISP has issues so no internet for me right now. I should be able to resume this weekend or next week.', DEFAULT, '2024-07-01 15:34:40', '2024-07-01 15:34:40');
INSERT INTO github.comments VALUES (2157078495, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2157078495', 'That is a really cool idea! We could add some kind of keyword like:`LSP-AI Tag` and then search for it in the current file.

A user working on a Python file could use it like so:
```
# LSP-AI Tag: This file is implementing a fibonnaci sequence 

def fib
```

Then it could become a variable they can use when building the prompt. 

E.G.

```
{
  "messages": [
    {
      "role":"system",
      "content":"You are a programming completion tool. Replace <CURSOR> with the correct code."
    },
    {
      "role": "user",
      "content": "{LSP-AI_TAG}\n{CODE}"
    }
  ]
}
```', DEFAULT, '2024-06-10 02:27:29', '2024-06-10 02:27:42');
INSERT INTO github.comments VALUES (2158552756, 2341547917, 50424412, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2158552756', '> We want to make it as easy as possible for everyone to get started using LSP-AI but I think it requires they make some initial decision on at least what backend they want to use which brings me back to being unsure on how to implement any default config.

... and ...

> For the VS Code plugin, I thought that making OpenAI with gpt-4o the default in the plugin settings would be a reasonable choice, but it honestly wasn''t my favorite as it still requires users to set an OPENAI_API_KEY for the plugin to work.

I think this is the best case for a default config. It''s much more likely that a user will expose an `OPENAI_API_KEY` than have set up a local LLM already; especially we wouldn''t know what model they''re running. So I''m in favor of using openai as the default config.', DEFAULT, '2024-06-10 14:40:53', '2024-06-10 14:40:53');
INSERT INTO github.comments VALUES (2159384318, 2341547917, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2159384318', '> For "ghost text" I am working on features for [supermaven-nvim](https://github.com/supermaven-inc/supermaven-nvim) and we use something like this:
> 
> * https://github.com/supermaven-inc/supermaven-nvim/blob/ef3bd1a6b6f722857f2f88d929dd4ac875655611/lua/supermaven-nvim/completion_preview.lua#L59-L88
> 
> I didn''t implement this feature so I am not gonna say I completely get it but when creating the `autocmd` adding a namespace like `lsp-ai` for example and when getting the result add it to a `api-extended-marks`, RTFM for more information on that.
> 
> ```lua
> local augroup = vim.api.nvim_create_augroup("lsp-ai", { clear = true })
> local ns_id = vim.api.nvim_create_namespace("lsp-ai")
> local opts = {
>   id = 1,
>   hl_mode = "combine",
> }
> 
> vim.api.nvim_create_autocmd({"BufEnter", "BufWinEnter"}, {
>   group = augroup,
>   callback = function() vim.lsp.start(lsp_ai_config) end,
> })
> 
> -- Register key shortcut
> vim.keymap.set(
>     "n", 
>     "<leader>co", 
>     function()
>         print("Loading completion...")
> 
>         local x = vim.lsp.util.make_position_params(0)
>         local y = vim.lsp.util.make_text_document_params(0)
> 
>         local combined = vim.tbl_extend("force", x, y)
> 
>         local result = vim.lsp.buf_request_sync(
>             0,
>             "textDocument/completion",
>             combined,
>             10000
>         )
> 
>         print(vim.inspect(result))
>         vim.api.nvim_buf_set_extmark(0, ns_id, vim.fn.line(".") - 1, vim.fn.col(".") - 1, opts)
>     end,
>     {
>         noremap = true,
>     }
> )
> ```
> 
> Warning
> 
> Not sure this works as expected for ghost text or how the `result` comes in. So you may have to modified and play around a bit.

Thanks for sharing!', DEFAULT, '2024-06-10 22:03:35', '2024-06-10 22:03:35');
INSERT INTO github.comments VALUES (2168411830, 2341547917, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2168411830', '> @SilasMarvin, ok. Will start working on it tomorrow. Are you ok with name `lsp-ai.nvim`. Do you suggest any other name :D?

That is a great name for it I love it! Let me know how it goes excited to see it!', DEFAULT, '2024-06-14 16:46:59', '2024-06-14 16:47:14');
INSERT INTO github.comments VALUES (2172374926, 2341547917, 2666479, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2172374926', 'Can anyone help me to test this `https://github.com/SuperBo/lsp-ai.nvim`.

Example config lazy config can be like this:

```lua
  {
    ''SuperBo/lsp-ai.nvim'',
    opts = {
      -- autostart = false,
      server = {
        memory = {
          file_store = {},
        },
        models = {
          model1 =  {
            type="llama_cpp",
            file_path="/opt/model/codeqwen-1_5-7b-chat-q4_k_m.gguf",
            n_ctx=512,
            -- ctx_size= 512,
            n_gpu_layers= 500,
          }
        }
      },
      generation = {
        model = "model1",
        parameters = {
          max_tokens=256,
          max_context=1024,
          messages = {
            {
              role="system",
              content="You are a programming completion tool. Replace <CURSOR> with the correct code."
            },
            {
              role = "user",
              content = "{CODE}"
            }
          }
        }
      }
    },
    dependencies = { ''neovim/nvim-lspconfig'' },
  }
```

Command to ask LSP-AI is `LSPAIGenerate`.', DEFAULT, '2024-06-17 06:19:15', '2024-06-17 07:08:01');
INSERT INTO github.comments VALUES (2158883892, 2343972535, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2158883892', 'Just added a comment in this thread: https://github.com/SilasMarvin/lsp-ai/issues/2#issuecomment-2158816103

Excited to get this into nvim-lspconfig!', DEFAULT, '2024-06-10 17:04:35', '2024-06-10 17:04:35');
INSERT INTO github.comments VALUES (2157109353, 2342506239, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2157109353', 'This is a really interesting idea. Let me think on this some more and get back to you on it. I did just implement crawling of the code base in: https://github.com/SilasMarvin/lsp-ai/pull/14 Which will give us some good groundwork to build off of.

There definitely needs to be some thought put into how we build that kind of context for the model. Ideally it wouldn''t have to be something the user does manually. It could be part of an agent system where we have an LLM who''s job it is to tag files and build the context for the LLM to do completion?

I''ll think more on this. Really good suggestions thank you, I''m excited to work on this together!', DEFAULT, '2024-06-10 03:07:35', '2024-06-10 03:07:35');
INSERT INTO github.comments VALUES (2158642266, 2341547917, 304428, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2158642266', '> I think you are probably right here. I do think they shouldn''t be defaults on the server, they should be defaults for the config / plugin to send to the server. I don''t want to have defaults for the server that might expose users codebases to third parties.

Agree''d. I think ideally the LSP part and LSP/IDE glue should have sensible defaults, but the backend/model config should probably be left for the user to pick. So as far as I can tell, that leaves only the `memory` key, which accepts only an empty file store for now ? In this case aside from registering the `textDocument/generation` action with nvim, there  wouldn''t be that much to do.', DEFAULT, '2024-06-10 15:19:21', '2024-06-10 15:20:10');
INSERT INTO github.comments VALUES (2163146527, 2341547917, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2163146527', '> > Got it. I''ll ask in the matrix server.
> 
> Ok great!
> 
> > We could also have both?
> 
> Yes, in my understanding it''s not unusual in the nvim ecosystem to have an additional plugin for language server specific features while keeping the minimal config in the lspconfig plugin, it''s even the [official recommendation](https://github.com/neovim/nvim-lspconfig/blob/master/CONTRIBUTING.md#scope-of-lspconfig).

I asked on the Matrix, they recommended discussing it on their GitHub. I suggest we create a PR that requires the user to provide defaults and have a discussion about it in that PR.', DEFAULT, '2024-06-12 14:22:35', '2024-06-12 14:22:35');
INSERT INTO github.comments VALUES (2169027790, 2341547917, 2666479, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2169027790', '@AlejandroSuero, Thank you for good template, can I cherry pick your selene and stylua config?

For testing, I prefer native busted + [nlua](https://github.com/mfussenegger/nlua) setup. I also need to add [neorocks](https://github.com/nvim-neorocks/rocks.nvim) formula also. So I will start an empty repo first without any template. Hope that don''t bother you!', DEFAULT, '2024-06-15 01:34:13', '2024-06-15 01:34:13');
INSERT INTO github.comments VALUES (2172463594, 2341547917, 2666479, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2172463594', '@fredrikaverpil, what language are you testing (python, go, ...). I''ve just hard code these support file types: `{ "go", "java", "python", "rust" }`. Will make this configurable later.

Do you have lsp-ai compiled with llama_cpp? You can test with openai if you have OPENAI key.

You don''t need to add config to `dependencies`.', DEFAULT, '2024-06-17 07:09:44', '2024-06-17 07:13:36');
INSERT INTO github.comments VALUES (2177554870, 2343972535, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2177554870', '> [PR](https://github.com/neovim/nvim-lspconfig/pull/3206) opened yesterday, took a bit longer than I would have liked but time is what it is. Waiting on me writing a bit of docs.

This looks awesome thank you! Let me know if I can assist at all', DEFAULT, '2024-06-19 03:52:13', '2024-06-19 03:52:13');
INSERT INTO github.comments VALUES (2313015516, 2488389244, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2313015516', 'This is a good point. I will make sure to use the release action going forward. https://github.com/SilasMarvin/lsp-ai/releases/tag/v0.6.2', DEFAULT, '2024-08-27 16:25:17', '2024-08-27 16:25:17');
INSERT INTO github.comments VALUES (2158452691, 2341547917, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2158452691', '> I''ll be opening a draft PR in a bit. I would not recommend merging it until the default config is integrated in `nvim-lspconfig` (managing the LSP lifecycle by hand is annoying and exactly what `nvim-lspconfig` is here for) but at least it should give a place to point the more adventurous people that want to try it _right now_.

This is awesome!

One thing I''m still unsure of is how to handle default configs. Right now if a user passes empty `initializationOptions` to LSP-AI we error. We require they provide a `memory` object and `models` array. 

We could absolutely provide a default on the server for `memory`, but its tough choosing the default `models` array as there are many options for model backends, and all are hardware or api key dependent.

For the VS Code plugin, I thought that making OpenAI with gpt-4o the default in the plugin settings would be a reasonable choice, but it honestly wasn''t my favorite as it still requires users to set an `OPENAI_API_KEY` for the plugin to work.

We want to make it as easy as possible for everyone to get started using LSP-AI but I think it requires they make some initial decision on at least what backend they want to use which brings me back to being unsure on how to implement any default config.', DEFAULT, '2024-06-10 13:58:03', '2024-06-10 13:58:29');
INSERT INTO github.comments VALUES (2159280095, 2341547917, 304428, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2159280095', '> Got it. I''ll ask in the matrix server.

Ok great!

> We could also have both?

Yes, in my understanding it''s not unusual in the nvim ecosystem to have an additional plugin for language server specific features while keeping the minimal config in the lspconfig plugin, it''s even the [official recommendation](https://github.com/neovim/nvim-lspconfig/blob/master/CONTRIBUTING.md#scope-of-lspconfig).', DEFAULT, '2024-06-10 21:03:51', '2024-06-10 21:03:51');
INSERT INTO github.comments VALUES (2168409721, 2341547917, 2666479, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2168409721', '@SilasMarvin, ok. Will start working on it tomorrow. Are you ok with name `lsp-ai.nvim`. Do you suggest any other name :D?', DEFAULT, '2024-06-14 16:45:41', '2024-06-14 16:45:41');
INSERT INTO github.comments VALUES (2171752081, 2341547917, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2171752081', 'That is awesome!!! I love it. This is really exciting stuff!', DEFAULT, '2024-06-16 15:58:07', '2024-06-16 15:58:07');
INSERT INTO github.comments VALUES (2158814151, 2343972535, 304428, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2158814151', 'I added an lspconfig based example using [my fork](https://github.com/Robzz/nvim-lspconfig) (happy to send the PR their way once we''re happy with it).', DEFAULT, '2024-06-10 16:24:07', '2024-06-10 16:24:07');
INSERT INTO github.comments VALUES (2293755313, 2468630958, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2293755313', 'Thanks for this PR. I will look into it this weekend, but it looks really good!', DEFAULT, '2024-08-16 15:53:18', '2024-08-16 15:53:18');
INSERT INTO github.comments VALUES (2158602315, 2341547917, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2158602315', '> > We want to make it as easy as possible for everyone to get started using LSP-AI but I think it requires they make some initial decision on at least what backend they want to use which brings me back to being unsure on how to implement any default config.
> 
> ... and ...
> 
> > For the VS Code plugin, I thought that making OpenAI with gpt-4o the default in the plugin settings would be a reasonable choice, but it honestly wasn''t my favorite as it still requires users to set an OPENAI_API_KEY for the plugin to work.
> 
> I think this is the best case for a default config. It''s much more likely that a user will expose an `OPENAI_API_KEY` than have set up a local LLM already; especially we wouldn''t know what model they''re running. So I''m in favor of using openai as the default config.

I think you are probably right here. I do think they shouldn''t be defaults on the server, they should be defaults for the config / plugin to send to the server. I don''t want to have defaults for the server that might expose users codebases to third parties.', DEFAULT, '2024-06-10 15:01:27', '2024-06-10 15:01:27');
INSERT INTO github.comments VALUES (2159390003, 2341547917, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2159390003', '> > Got it. I''ll ask in the matrix server.
> 
> Ok great!
> 
> > We could also have both?
> 
> Yes, in my understanding it''s not unusual in the nvim ecosystem to have an additional plugin for language server specific features while keeping the minimal config in the lspconfig plugin, it''s even the [official recommendation](https://github.com/neovim/nvim-lspconfig/blob/master/CONTRIBUTING.md#scope-of-lspconfig).

Got it that makes sense. I''ll ask around about defaults in the matrix probably tomorrow.

Would you want to head up our neovim plugin? I''m thinking for now we just fork llm-ls, edit the configuration options to match the options I have for our VS Code plugin, and then just have it perform inline completion with ghost text. ', DEFAULT, '2024-06-10 22:08:06', '2024-06-10 22:08:06');
INSERT INTO github.comments VALUES (2168690263, 2341547917, 71392160, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2168690263', '@SuperBo @SilasMarvin I created this [template repo](https://github.com/AlejandroSuero/template.nvim) for Neovim plugins, it has `.editorconfig`, `selene` and `stylua` ready to go with easy to use `make` targets and CI.

It also has `plenary.nvim` and `vusted` tests set up and ready to use with `make` targets and CI.

It also has more utilities but that is more of a personal opinion on why you should use something like `codespell` but you can always ignore it and delete it.', DEFAULT, '2024-06-14 20:09:44', '2024-06-14 20:09:44');
INSERT INTO github.comments VALUES (2172437467, 2341547917, 994357, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2172437467', 'I had to do these modifications:

```diff
  {
-    dir =''SuperBo/lsp-ai.nvim'',
+    "SuperBo/lsp-ai.nvim",
    opts = {
      -- autostart = false,
      server = {
        memory = {
          file_store = {},
        },
        models = {
          model1 =  {
            type="llama_cpp",
            file_path="/opt/model/codeqwen-1_5-7b-chat-q4_k_m.gguf",
            n_ctx=512,
            -- ctx_size= 512,
            n_gpu_layers= 500,
          }
        }
      },
      generation = {
        model = "model1",
        parameters = {
          max_tokens=256,
          max_context=1024,
          messages = {
            {
              role="system",
              content="You are a programming completion tool. Replace <CURSOR> with the correct code."
            },
            {
              role = "user",
              content = "{CODE}"
            }
          }
        }
      }
    },
    dependencies = { ''neovim/nvim-lspconfig'' },
+     config = function(_, opts)
+       require("lsp_ai").setup(opts)
+     end,
  }
```

But I can''t run `:LSPAIGenerate`:

```
   Error  08:52:51 msg_show.emsg   LSPAIGenerate E492: Not an editor command: LSPAIGenerate
```', DEFAULT, '2024-06-17 06:54:40', '2024-06-17 06:54:40');
INSERT INTO github.comments VALUES (2174843031, 2343972535, 304428, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2174843031', '[PR](https://github.com/neovim/nvim-lspconfig/pull/3206) opened yesterday, took a bit longer than I would have liked but time is what it is. Waiting on me writing a bit of docs.', DEFAULT, '2024-06-18 02:36:51', '2024-06-18 14:28:11');
INSERT INTO github.comments VALUES (2156807102, 2342506239, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156807102', 'Thank for getting back so quickly!

I''ve already installed the plugin and tried both llama & open ai but neither worked. I wrote a comment, then moved my cursor down and invoked the LSP-AI generate but nothing happens.', DEFAULT, '2024-06-09 21:58:21', '2024-06-09 21:58:21');
INSERT INTO github.comments VALUES (2159257860, 2341547917, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2159257860', '> > Or maybe I am misunderstanding what is standard for nvim-lspconfig and its ok to provide a default that doesn''t full work without the user providing more parameters?
> 
> This I really don''t know, maybe there are similar cases among the supported LSP servers in nvim-lspconfig, I haven''t seen any but I only glanced over it and it''s a _long_ list. I haven''t seen any AI LSP servers in there either since HuggingFace have gone the way of writing their own plugin instead for `llm-ls` so there''s really no one to copy from. Asking on the nvim Matrix server is probably the easiest way to know for sure.

Got it. I''ll ask in the matrix server. 

We could do the same thing llm-ls does and just fork the llm-ls neovim plugin. This would provide a better user experience for completions. I know eventually we do want our own plugin.

We could also have both?', DEFAULT, '2024-06-10 20:48:38', '2024-06-10 20:48:38');
INSERT INTO github.comments VALUES (2167603020, 2341547917, 2666479, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2167603020', 'Hi @SilasMarvin, @Robzz, what is you guys final decision. If you decided to start a dedicated plugin, I''m happy to help (FYI, I''m developing another Neovim plugin https://github.com/SuperBo/fugit2.nvim). ', DEFAULT, '2024-06-14 09:13:28', '2024-06-14 09:13:59');
INSERT INTO github.comments VALUES (2169068390, 2341547917, 71392160, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2169068390', '@SuperBo yeah, go for it.

I will be checking out your git plugin tomorrow and also get a look for the testing setup. ', DEFAULT, '2024-06-15 02:51:30', '2024-06-15 02:51:30');
INSERT INTO github.comments VALUES (2195946623, 2341547917, 2666479, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2195946623', '@SilasMarvin, sorry, I''ve been quite busy since last week. I will have more free time this weekend. See you in discord.', DEFAULT, '2024-06-28 01:40:37', '2024-06-28 01:40:37');
INSERT INTO github.comments VALUES (2156226658, 2341796211, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2156226658', 'I will check it out thank you! I''ll also update this thread soon with some suggestions for chunk formats and potentially defaults for embedding models, etc...', DEFAULT, '2024-06-08 23:36:59', '2024-06-08 23:36:59');
INSERT INTO github.comments VALUES (2168345665, 2341547917, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2168345665', '> Hi @SilasMarvin, @Robzz, what is you guys final decision. If you decided to start a dedicated plugin, I''m happy to help (FYI, I''m developing another Neovim plugin https://github.com/SuperBo/fugit2.nvim).

We definitely want to have a dedicated plugin, if you want to get started on it that would be awesome! We have one for VS Code that should be a good reference: https://github.com/SilasMarvin/lsp-ai/tree/main/editors/vscode Here is an overview on the wiki about it: https://github.com/SilasMarvin/lsp-ai/wiki/Plugins

You could also fork: https://github.com/huggingface/llm.nvim and use it as a base. I''m happy to give more input if you want, just let me know! ', DEFAULT, '2024-06-14 16:13:49', '2024-06-14 16:13:49');
INSERT INTO github.comments VALUES (2171751150, 2341547917, 2666479, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2171751150', '
https://github.com/SilasMarvin/lsp-ai/assets/2666479/5d40b301-ac03-4788-a3b6-9306344c34e8

First update guys, we now can ask AI for whole file code completion.', DEFAULT, '2024-06-16 15:54:21', '2024-06-16 15:54:21');
INSERT INTO github.comments VALUES (2157152693, 2342658847, 16382165, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2157152693', '@SilasMarvin,
Crawling could suck for legacy projects as they might have huge codebases. One of my previous companies I worked on a project that was almost 15 years old. It was .NET so I had to restart the server every time I made a change. The feedback loop was painful.

I was on a decent system too. Imagine the user is working on ML project where they''re also running a local model which requires compute resources. Might increase their startup time a lot too =).

Regardless, I''d like ask/suggest  LSP''s [workspace get configuration](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_configuration) method here. 

Do you think we could read configs for the project instead of crawling? As a specification, I think we''re free to make it work or add additional APIs if we want; right? We might have to do both long term to build a Co pilot Killer so I''m more interested in understanding your thoughts on the LSP spec here =).
', DEFAULT, '2024-06-10 03:56:16', '2024-06-10 03:58:34');
INSERT INTO github.comments VALUES (2275927081, 2341796211, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2275927081', 'Closing this as its now complete as of 0.4.0', DEFAULT, '2024-08-08 14:08:10', '2024-08-08 14:08:10');
INSERT INTO github.comments VALUES (2313165338, 2488389244, 44335182, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2313165338', '> This is a good point. I will make sure to use the release action going forward. https://github.com/SilasMarvin/lsp-ai/releases/tag/v0.6.2

Thank you!', DEFAULT, '2024-08-27 17:43:00', '2024-08-27 17:43:00');
INSERT INTO github.comments VALUES (2370243482, 2537650065, 19626586, 'https://api.github.com/repos/SilasMarvin/lsp-ai/issues/comments/2370243482', '> > > This is a really awesome pr! I''ve used nix a little bit but not enough to know the proper ways to do things. Ss there some kind of way to make sure this builds correctly in our CI?
> > 
> > 
> > I am not sure I understand: Is there a desire to use nix instead for CI? As these changes aren''t _necessary_ or intended to replace a working CI, unless it''s needed.
> 
> I think he''s referring to ensuring that the nix package builds (which theoretically could fail even if the rust package can compile via the normal imperative commands). There seems to be [some documentation](https://docs.cachix.org/continuous-integration-setup/) already over using nix in CI, ~and if there are no immediate plans to merge into the `nixpkgs` repo~ (updated based on latest comment), there''s an [official tutorial](https://nix.dev/guides/recipes/continuous-integration-github-actions) on using GitHub Actions to automatically push binaries to a Cachix binary cache (to prevent everyone building the same derivation on their own systems) if this project were to adopt one.

That is exactly what I am referring to. I''m a bit worried to merge this in without actually using nix myself and having no way to verify that it is consistently working. I know a lot of people do use it and I would love to have a flake, but I want to make sure it is sustainable.', DEFAULT, '2024-09-24 05:45:33', '2024-09-24 05:45:33');


--
-- Data for Name: issue_assignees; Type: TABLE DATA; Schema: github; Owner: silas
--

INSERT INTO github.issue_assignees VALUES (2456334467, 19626586);
INSERT INTO github.issue_assignees VALUES (2453150358, 19626586);
INSERT INTO github.issue_assignees VALUES (2451215027, 19626586);
INSERT INTO github.issue_assignees VALUES (2342117668, 19626586);


--
-- Data for Name: labels; Type: TABLE DATA; Schema: github; Owner: silas
--

INSERT INTO github.labels VALUES (6248202382, 'enhancement', 'New feature or request');
INSERT INTO github.labels VALUES (7188086985, 'discussion', '');
INSERT INTO github.labels VALUES (6248202368, 'bug', 'Something isn''t working');
INSERT INTO github.labels VALUES (6248202371, 'documentation', 'Improvements or additions to documentation');
INSERT INTO github.labels VALUES (6248202398, 'question', 'Further information is requested');
INSERT INTO github.labels VALUES (7063125156, 'in review', 'This pull request is in review');


--
-- Data for Name: pull_requests; Type: TABLE DATA; Schema: github; Owner: silas
--

INSERT INTO github.pull_requests VALUES (2389880446, '2024-07-27 18:46:42');
INSERT INTO github.pull_requests VALUES (2375935712, NULL);
INSERT INTO github.pull_requests VALUES (2374752627, '2024-06-27 06:06:48');
INSERT INTO github.pull_requests VALUES (2366824348, '2024-06-25 04:26:25');
INSERT INTO github.pull_requests VALUES (2357003247, '2024-06-21 15:15:56');
INSERT INTO github.pull_requests VALUES (2352266185, '2024-06-14 00:39:29');
INSERT INTO github.pull_requests VALUES (2346771112, '2024-06-11 15:52:06');
INSERT INTO github.pull_requests VALUES (2343972535, NULL);
INSERT INTO github.pull_requests VALUES (2343344751, '2024-06-12 23:58:12');
INSERT INTO github.pull_requests VALUES (2342658847, NULL);
INSERT INTO github.pull_requests VALUES (2342373064, '2024-06-09 16:37:34');
INSERT INTO github.pull_requests VALUES (2342369376, '2024-06-09 16:28:18');
INSERT INTO github.pull_requests VALUES (2342327300, '2024-06-09 15:06:47');
INSERT INTO github.pull_requests VALUES (2342112427, '2024-06-09 08:59:32');
INSERT INTO github.pull_requests VALUES (2545637874, '2024-09-24 14:51:29');
INSERT INTO github.pull_requests VALUES (2539087774, '2024-09-20 15:22:41');
INSERT INTO github.pull_requests VALUES (2538911233, '2024-09-20 15:22:00');
INSERT INTO github.pull_requests VALUES (2537650065, NULL);
INSERT INTO github.pull_requests VALUES (2508058822, '2024-09-05 15:04:46');
INSERT INTO github.pull_requests VALUES (2493688688, '2024-09-05 14:35:26');
INSERT INTO github.pull_requests VALUES (2489825675, '2024-08-27 16:13:59');
INSERT INTO github.pull_requests VALUES (2478376063, '2024-08-21 15:40:22');
INSERT INTO github.pull_requests VALUES (2471642512, '2024-08-21 02:55:54');
INSERT INTO github.pull_requests VALUES (2468630958, NULL);
INSERT INTO github.pull_requests VALUES (2459766205, '2024-08-11 21:20:30');
INSERT INTO github.pull_requests VALUES (2459757335, '2024-08-11 20:53:42');
INSERT INTO github.pull_requests VALUES (2456111460, '2024-08-08 15:37:55');
INSERT INTO github.pull_requests VALUES (2456106129, '2024-08-08 15:35:09');
INSERT INTO github.pull_requests VALUES (2451504473, '2024-08-06 19:42:27');
INSERT INTO github.pull_requests VALUES (2451037524, '2024-08-06 14:29:27');
INSERT INTO github.pull_requests VALUES (2449871303, '2024-08-06 03:43:17');
INSERT INTO github.pull_requests VALUES (2449859817, '2024-08-06 03:30:56');
INSERT INTO github.pull_requests VALUES (2449858683, '2024-08-06 03:29:36');
INSERT INTO github.pull_requests VALUES (2449703999, '2024-08-06 02:56:43');
INSERT INTO github.pull_requests VALUES (2204079308, '2024-03-24 02:02:47');


--
-- PostgreSQL database dump complete
--

