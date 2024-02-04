import json
import os


def check_keys(template_file, language_files):
    template_data = json.load(open(template_file, 'r', encoding='utf-8'))

    for language_file in language_files:
        language_data = json.load(open(language_file, 'r', encoding='utf-8'))

        # 检查模板文件中的key是否都在语言文件中
        for key in sorted(template_data.keys()):
            if key not in language_data:
                print(
                    f"Key '{key}' not found in language file '{language_file}'")
                language_data[key] = template_data[key]

        # 检查语言文件中的key是否都有对应的值
        for key in sorted(language_data.keys()):
            if key not in template_data:
                print(
                    f"Key '{key}' found in language file '{language_file}' but not in template")
                del language_data[key]

        # 将语言文件的key按照模版文件的顺序进行排列
        language_data_sorted = {}
        for key in template_data.keys():
            if key in language_data:
                language_data_sorted[key] = language_data[key]

        with open(language_file, 'w', encoding='utf-8') as file:
            json.dump(language_data_sorted, file, ensure_ascii=False, indent=2)

####################
# This is a tool script for updating language.
# If there are new keys to be added, you can directly add them in template.json.
# At the same time, the sorting of keys in the language file will also match template.json.
# If you want to delete a key, you can directly remove it from template.json and then run the script.
####################


if __name__ == "__main__":
    # Here is the template file
    template_file = "template.json"

    # Here is language files that need to be updated
    language_files = ["chinese.json", "english.json"]

    if not os.path.exists(template_file):
        print(f"{template_file} not found")
        exit(1)

    for language_file in language_files:
        if not os.path.exists(language_file):
            print(f"{language_file} not found, creating a new file")
            with open(language_file, 'w', encoding='utf-8') as file:
                file.write("{}")

    check_keys(template_file, language_files)
    print("Key checking, sorting, and updating completed")
