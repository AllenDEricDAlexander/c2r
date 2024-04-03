from zhipuai import ZhipuAI

client = ZhipuAI(api_key="8272b05b9d4509f7035309fd5cc5f364.Lyv5Es6cufdBx0bn")


def chat(prompt) -> str:
    return client.chat.completions.create(
        model="GLM-3-Turbo",
        messages=prompt
    ).choices[0].message.content
