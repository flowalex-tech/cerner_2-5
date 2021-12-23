#!/usr/bin/env python3
#cerner_2^5_2020
#Goal: Print Address with POSTNET bar code. (POSTNET means Postal Numeric Encoding Technique.)
# Based off of this java code I wrote in school https://gitlab.com/flowalex/code-practice/-/blob/mainline/java/postnet.java
import csv
from collections import defaultdict
columns = defaultdict(list)
with open("addresses.txt") as csvfile:
    reader = csv.reader(csvfile, delimiter=",",quotechar='"')
    for row in reader:
        name = row[0]
        address = row[1]
        city = row[2]
        state = row[3]
        zip_code= row[4]
        print(name+"\n"+address+"\n"+city+" "+state+" "+zip_code)
        barcode=[":::||","::|:|","::||:",":|::|",":|:|:",":||::","|:::|","|::|:","|:|::","||:::"]
        zip_list=[]
        postnet_array=[]
        ZipCode=zip_code.replace("-", "")
        for elem in ZipCode:
            zip_list.append(int(elem))
            new_elem=int(elem)
            for i  in range(10):
                if new_elem == i:
                    postnet_array.append(barcode[i])
        checksum_add=sum(zip_list)
        checksum_mod=checksum_add %10
        checksum=10-checksum_mod
        print("|"+postnet_array[0]+postnet_array[1]+postnet_array[2]+postnet_array[3]+postnet_array[4]+postnet_array[5]+postnet_array[6]+postnet_array[7]+postnet_array[8]+barcode[checksum]+"|"+"\n\n")
        f=open("labels.txt","a")
        f.write(name+"\n"+address+"\n"+city+" "+state+" "+zip_code+"\n")
        f.write("|"+postnet_array[0]+postnet_array[1]+postnet_array[2]+postnet_array[3]+postnet_array[4]+postnet_array[5]+postnet_array[6]+postnet_array[7]+postnet_array[8]+barcode[checksum]+"|"+"\n\n")
        f.close
