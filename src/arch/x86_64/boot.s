/* Multiboot header */

/* Constants */
.set MAGIC,         0xE85250D6
.set ARCHITECTURE,  0
.set CHECKSUM,      1 << 32 - MAGIC - ARCHITECTURE

/* Multiboot struct */
.section .multiboot
.align  4
.long   MAGIC
.long   ARCHITECTURE
.long   0
.long   CHECKSUM

/* Terminator tag */
.align  8
.short  0
.short  0
.long   8




.section .bss
/* paging */
.align 4096
p4_table:
.skip 4096
p3_table:
.skip 4096
p2_table:
.skip 4096
/* Stack */
stack_bottom:
.skip 16384 # 16 Kib
stack_top:



/* Call kernel_main */
.section .text
.code32
.global _start
.type _start, @function
_start:
    // initialise stack
    mov $stack_top, %esp
    // pass multiboot info to kernel_main function
    mov %ebx, %edi

    call check_multiboot
    call check_cpuid
    call check_long_mode

    call set_up_page_tables
    call enable_paging

    lgdt (gdt64_pointer)

    // long jump to long mode
    ljmp  $gdt64_code,$long_mode_start

1:  hlt                         // loop
    jmp 1b


/* check if load from multiboot */
check_multiboot:
    cmp $0x36d76289, %eax
    jne .no_multiboot
    ret
.no_multiboot:
    mov $0x4f30, %al
    jmp error


/* check if cpuid is supported by the cpu for retrieved some informations */
check_cpuid:
    pushfd
    pop %eax
    mov %eax, %ecx
    xor $(1 << 21), %eax
    push %eax
    popfd
    pushfd
    pop %eax
    push %ecx
    popfd
    cmp %eax, %ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov $0x4f31, %al
    jmp error


check_long_mode:
    mov $0x80000000, %eax
    cpuid
    cmp $0x80000001, %eax
    jb  .no_long_mode

    mov $0x80000001, %eax
    cpuid
    test $(1 << 29),%edx
    jz  .no_long_mode
    ret


.no_long_mode:
    mov $0x4f32, %al
    jmp error

error:
    movl $0x4f524f45, (0xb8000)
    movl $0x4f3a4f52, (0xb8004)
    movl $0x4f204f20, (0xb8008)
    movb %al, (0xb800a)
    hlt




set_up_page_tables:
    mov $p3_table, %eax
    or $0b11, %eax
    mov %eax, (p4_table)

    mov $p2_table, %eax
    or $0b11, %eax
    mov %eax, (p3_table)

    mov $0, %ecx

.map_p2_table:

    mov $0x200000, %eax   // 2MiB
    mul %ecx
    or $0b10000011, %eax
    mov %eax, p2_table( , %ecx, 8)

    inc %ecx
    cmp $512, %ecx
    jne .map_p2_table

    ret

enable_paging:
    mov $p4_table, %eax
    mov %eax, %cr3

    mov %cr4, %eax
    or $(1 << 5), %eax
    mov %eax, %cr4

    mov $0xC0000080, %ecx
    rdmsr
    or $(1 << 8), %eax
    wrmsr

    mov %cr0, %eax
    or $(1 << 31), %eax
    mov %eax, %cr0

    ret


.section .rodata
gdt64:
.quad 0
.equ gdt64_code,  . - gdt64
.quad ( (1 << 43 ) | ( 1 << 44 ) | (1 << 47) | (1 << 53)) // code segment
gdt64_pointer:
.word . - gdt64 - 1
.quad gdt64



.section .text
.code64
long_mode_start:
    // reset register
    mov $0, %ax
    mov %ax, %ss
    mov %ax, %ds
    mov %ax, %es
    mov %ax, %fs
    mov %ax, %gs

    // check point : print OKAY in VGA Buffer
    mov $0x2f592f412f4b2f4f, %rax
    mov %rax, (0xb8000)

    // call main function in rust
    call kernel_main

1:  hlt                         // loop
    jmp 1b


.size _start, . - _start
