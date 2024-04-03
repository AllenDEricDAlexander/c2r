import os


def r_stream(file_path, r_code):
    os.makedirs(os.path.dirname(file_path), exist_ok=True)
    with open(file_path, 'w+') as file:
        # 读取文件内容
        file.write(r_code)
