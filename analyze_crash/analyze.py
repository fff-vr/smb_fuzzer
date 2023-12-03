import glob

for filepath in glob.glob('../crashlog/*.txt'):
    with open(filepath, 'r', encoding='utf-8', errors='ignore') as file:
        lines = file.readlines()
        found = False  # 문자열을 찾았는지 여부를 추적하는 플래그
        ignore = False
        save_line=""
        for i, line in enumerate(lines):
            if "------------[ cut here ]------------" in line:
                # 다음 줄이 파일의 마지막 줄이 아니면 출력
                if i + 1 < len(lines):
                    #print(f"{filepath}  : {lines[i + 1].strip()}")
                    found = True
                    save_line=lines[i+1]
            elif "BUG:" in line:
                #print(f"{filepath}  : {line.strip()}")
                found = True
                save_line=line
            elif "smb2_parse_contexts" in line:
                ignore=True
            elif "parse_server_interfaces" in line:
                ignore=True
            #elif "cifs_limit_bvec_subset" in line:
            #    ignore=True
            #elif "smb2_query_symlink" in line:
            #    ignore= True
            #elif "cifs_small_buf_get" in line:
            #    ignore=True
            #elif "qlist_free_all" in line:
            #    ignore = True
        if found and not ignore:
            print(f"{filepath}: {save_line}")
            # 어떤 문자열도 찾지 못한 경우
            #print(f"{filepath}: No 'cut here' or 'BUG:' found")

