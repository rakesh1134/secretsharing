# secretsharing
one-time pad example

Input file content is xor'd with a sequence of random bytes. The program creates 2 files. First file contains the random bytes(key) used to encrypt input file.
Second file contains the encyrpted data. 
To recreate the original file, perform xor on the conents of first & second files.

