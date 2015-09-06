section .data
section .text
global  _start                ; точка входа в программу
_start:
    mov eax, 1                ; системный вызов № 1 — sys_exit
    xor ebx, ebx              ; выход с кодом 0
    int 80h                   ; вызов ядра
