import subprocess


def compiler_rust(file_path) -> str:
    command = "rustc " + file_path

    try:
        result = subprocess.run(command, shell=True, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                text=True)
        return "True"
    except subprocess.CalledProcessError as e:
        print("命令执行失败。错误信息：")
        return e.stderr
