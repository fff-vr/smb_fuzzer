for file in workdir/save/*.txt; do
    #bug1
    if ! grep -q "smb2_parse_contexts" "$file"; then
        #bug2
        if ! grep -q "cifs_put_tcp_session" "$file"; then
            echo "$file"
        fi
    fi
done

