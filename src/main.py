from chat.model import GLM
from tools import read_c, write_r, parser, test_command

c_path = 'c_code/medium-level/shm/client.c'
r_path = 'r_code/medium-level/shm/client.rs'

code = read_c.c_stream(c_path)

code_c = code + "以上是你需要转译的代码，请保证输出仅有Rust语言代码，不要做出任何解释。"

prompt_message = [
    {"role": "system",
     "content": "你是一个优秀的代码转译能力者，接下来你的工作是将用户发送给你的C/C++语言代码，转译成含义相同，且能正常通过编译的Rust语言代码，保证两者等价,"
                "请注意对宏的处理。保证输出仅有Rust语言代码，不要做出任何解释。"},
    {"role": "user", "content": code_c}
]

response = GLM.chat(prompt_message)

# print(response)

response = parser.parser_content(response)

write_r.r_stream(r_path, response)

result = test_command.compiler_rust(r_path)
