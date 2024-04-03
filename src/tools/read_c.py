def c_stream(file_path) -> str:
    with open(file_path, 'rb') as file:
        # 读取文件内容
        file_content = file.read().decode('utf-8')
        return file_content
