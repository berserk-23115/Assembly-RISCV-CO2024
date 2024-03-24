def compare_files(file1, file2):
    with open(file1, "r") as f1, open(file2, "r") as f2:
        lines1 = f1.readlines()
        lines2 = f2.readlines()

    num_lines = min(len(lines1), len(lines2))
    for i in range(num_lines):
        if lines1[i] != lines2[i]:
            print(f"Difference found at line {i+1}:")
            print(f"{file1}: {lines1[i]}")
            print(f"{file2}: {lines2[i]}")
            return

    if len(lines1) != len(lines2):
        print("Files have different number of lines.")
    else:
        print("Files are identical.")


if __name__ == "__main__":
    file1 = "output.txt"
    file2 = "actual_output.txt"
    compare_files(file1, file2)
