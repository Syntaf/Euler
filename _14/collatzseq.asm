; *****************************************************************
;  Data Declarations 
section .data

; -----
;  Standard constants.

TRUE        equ 1
FALSE       equ 0

EXIT_SUCCESS    equ 0           ; successful operation
SYS_exit    equ 60              ; call code for terminate

maxChainLength  dq  0
maxValue        dq  0  

; *****************************************************************
;  Code Section

section .text
global _start
_start:

; Find the number that procudes that largest Collatz sequence

    mov r9, 2               ; shortcut for mul by 2
    mov r10, 3              ; shortcut for mul by 3
    mov r11, 0              ; count of seqence

    mov rcx, 999999         ; loop counter
    collatz:
        mov rax, rcx        ; move current number to rax
        mov r11, 0          ; reset sequence count to 0
        sequence:
            mov r12, rax    ; save state of rax
            mov rdx, 0      ; set forward 64 bits to zero
            div r9          ; divide by 2

            cmp rdx, 0      ; is even? if yes skip
            je isEven
                mov rax, r12; pop saved state back into rax
                mul r10     ; multiply by 3
                add rax, 1  ; add 1
            isEven:
    
            inc r11         ; increment sequence
            
            cmp rax, 1      ; if final value is 1, end
            jne sequence

        ; compare current sequence to max sequence, if bigger
        ; set new sequence
        cmp r11, qword[maxChainLength]  
        jle skipSetMax      
            mov qword[maxChainLength], r11
            mov qword[maxValue], rcx
        skipSetMax:
    loop collatz

; *****************************************************************
;   Done, terminate program.

last:
    mov eax, SYS_exit       ; call code for exit
    mov ebx, EXIT_SUCCESS   ; exit program with success
    syscall
