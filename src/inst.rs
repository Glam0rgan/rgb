use crate::cpu::Cpu;
use crate::mmu::Mmu;

// ld b,b
#[allow(unused_variables)]
fn op_0040(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_b(v);

    (4, 1)
}

// ld b,c
#[allow(unused_variables)]
fn op_0041(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_b(v);

    (4, 1)
}

// ld b,d
#[allow(unused_variables)]
fn op_0042(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_b(v);

    (4, 1)
}

// ld b,e
#[allow(unused_variables)]
fn op_0043(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_b(v);

    (4, 1)
}

// ld b,h
#[allow(unused_variables)]
fn op_0044(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_b(v);

    (4, 1)
}

// ld b,l
#[allow(unused_variables)]
fn op_0045(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_b(v);

    (4, 1)
}

// ld b,a
#[allow(unused_variables)]
fn op_0047(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_b(v);

    (4, 1)
}

// ld c,b
#[allow(unused_variables)]
fn op_0048(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_c(v);

    (4, 1)
}

// ld c,c
#[allow(unused_variables)]
fn op_0049(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_c(v);

    (4, 1)
}

// ld c,d
#[allow(unused_variables)]
fn op_004a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_c(v);

    (4, 1)
}

// ld c,e
#[allow(unused_variables)]
fn op_004b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_c(v);

    (4, 1)
}

// ld c,h
#[allow(unused_variables)]
fn op_004c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_c(v);

    (4, 1)
}

// ld c,l
#[allow(unused_variables)]
fn op_004d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_c(v);

    (4, 1)
}

// ld c,a
#[allow(unused_variables)]
fn op_004f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_c(v);

    (4, 1)
}

// ld d,b
#[allow(unused_variables)]
fn op_0050(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_d(v);

    (4, 1)
}

// ld d,c
#[allow(unused_variables)]
fn op_0051(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_d(v);

    (4, 1)
}

// ld d,d
#[allow(unused_variables)]
fn op_0052(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_d(v);

    (4, 1)
}

// ld d,e
#[allow(unused_variables)]
fn op_0053(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_d(v);

    (4, 1)
}

// ld d,h
#[allow(unused_variables)]
fn op_0054(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_d(v);

    (4, 1)
}

// ld d,l
#[allow(unused_variables)]
fn op_0055(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_d(v);

    (4, 1)
}

// ld d,a
#[allow(unused_variables)]
fn op_0057(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_d(v);

    (4, 1)
}

// ld e,b
#[allow(unused_variables)]
fn op_0058(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_e(v);

    (4, 1)
}

// ld e,c
#[allow(unused_variables)]
fn op_0059(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_e(v);

    (4, 1)
}

// ld e,d
#[allow(unused_variables)]
fn op_005a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_e(v);

    (4, 1)
}

// ld e,e
#[allow(unused_variables)]
fn op_005b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_e(v);

    (4, 1)
}

// ld e,h
#[allow(unused_variables)]
fn op_005c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_e(v);

    (4, 1)
}

// ld e,l
#[allow(unused_variables)]
fn op_005d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_e(v);

    (4, 1)
}

// ld e,a
#[allow(unused_variables)]
fn op_005f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_e(v);

    (4, 1)
}

// ld h,b
#[allow(unused_variables)]
fn op_0060(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_h(v);

    (4, 1)
}

// ld h,c
#[allow(unused_variables)]
fn op_0061(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_h(v);

    (4, 1)
}

// ld h,d
#[allow(unused_variables)]
fn op_0062(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_h(v);

    (4, 1)
}

// ld h,e
#[allow(unused_variables)]
fn op_0063(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_h(v);

    (4, 1)
}

// ld h,h
#[allow(unused_variables)]
fn op_0064(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_h(v);

    (4, 1)
}

// ld h,l
#[allow(unused_variables)]
fn op_0065(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_h(v);

    (4, 1)
}

// ld h,a
#[allow(unused_variables)]
fn op_0067(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_h(v);

    (4, 1)
}

// ld l,b
#[allow(unused_variables)]
fn op_0068(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_l(v);

    (4, 1)
}

// ld l,c
#[allow(unused_variables)]
fn op_0069(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_l(v);

    (4, 1)
}

// ld l,d
#[allow(unused_variables)]
fn op_006a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_l(v);

    (4, 1)
}

// ld l,e
#[allow(unused_variables)]
fn op_006b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_l(v);

    (4, 1)
}

// ld l,h
#[allow(unused_variables)]
fn op_006c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_l(v);

    (4, 1)
}

// ld l,l
#[allow(unused_variables)]
fn op_006d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_l(v);

    (4, 1)
}

// ld l,a
#[allow(unused_variables)]
fn op_006f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_l(v);

    (4, 1)
}

// ld a,b
#[allow(unused_variables)]
fn op_0078(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_a(v);

    (4, 1)
}

// ld a,c
#[allow(unused_variables)]
fn op_0079(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_a(v);

    (4, 1)
}

// ld a,d
#[allow(unused_variables)]
fn op_007a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_a(v);

    (4, 1)
}

// ld a,e
#[allow(unused_variables)]
fn op_007b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_a(v);

    (4, 1)
}

// ld a,h
#[allow(unused_variables)]
fn op_007c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_a(v);

    (4, 1)
}

// ld a,l
#[allow(unused_variables)]
fn op_007d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_a(v);

    (4, 1)
}

// ld a,a
#[allow(unused_variables)]
fn op_007f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_a(v);

    (4, 1)
}

// ld a,(bc)
#[allow(unused_variables)]
fn op_000a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_bc());
    cpu.set_a(v);

    (8, 1)
}

// ld a,(de)
#[allow(unused_variables)]
fn op_001a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_de());
    cpu.set_a(v);

    (8, 1)
}

// ld b,(hl)
#[allow(unused_variables)]
fn op_0046(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_b(v);

    (8, 1)
}

// ld c,(hl)
#[allow(unused_variables)]
fn op_004e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_c(v);

    (8, 1)
}

// ld d,(hl)
#[allow(unused_variables)]
fn op_0056(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_d(v);

    (8, 1)
}

// ld e,(hl)
#[allow(unused_variables)]
fn op_005e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_e(v);

    (8, 1)
}

// ld h,(hl)
#[allow(unused_variables)]
fn op_0066(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_h(v);

    (8, 1)
}

// ld l,(hl)
#[allow(unused_variables)]
fn op_006e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_l(v);

    (8, 1)
}

// ld a,(hl)
#[allow(unused_variables)]
fn op_007e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_a(v);

    (8, 1)
}

// ld (bc),a
#[allow(unused_variables)]
fn op_0002(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_a();
    mmu.set8(cpu.get_bc(), v);

    (8, 1)
}

// ld (de),a
#[allow(unused_variables)]
fn op_0012(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_a();
    mmu.set8(cpu.get_de(), v);

    (8, 1)
}

// ld (hl),b
#[allow(unused_variables)]
fn op_0070(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_b();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

// ld (hl),c
#[allow(unused_variables)]
fn op_0071(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_c();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

// ld (hl),d
#[allow(unused_variables)]
fn op_0072(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_d();
    mmu.set8(cpu.get_bc(), v);

    (8, 1)
}

// ld (hl),e
#[allow(unused_variables)]
fn op_0073(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_e();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

// ld (hl),h
#[allow(unused_variables)]
fn op_0074(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_h();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

// ld (hl),l
#[allow(unused_variables)]
fn op_0075(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_l();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

// ld (hl),a
#[allow(unused_variables)]
fn op_0077(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_a();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

// ldi (hl),a
#[allow(unused_variables)]
fn op_0022(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_a();
    mmu.set8(cpu.get_hl(), v);

    cpu.set_hl(cpu.get_hl().wrapping_add(1));

    (8, 1)
}

// ldd (hl),a
#[allow(unused_variables)]
fn op_0022(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_a();
    mmu.set8(cpu.get_hl(), v);

    cpu.set_hl(cpu.get_hl().wrapping_sub(1));

    (8, 1)
}

// ldi a,(hl)
#[allow(unused_variables)]
fn op_002a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_a(v);

    cpu.set_hl(cpu.get_hl().wrapping_add(1));

    (8, 1)
}

// ldd a,(hl)
#[allow(unused_variables)]
fn op_003a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_a(v);

    cpu.set_hl(cpu.get_hl().wrapping_sub(1));

    (8, 1)
}

// ld (0xff00+c),a
#[allow(unused_variables)]
fn op_00e2(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_a();
    mmu.set8(0xff00 + cpu.get_c() as u16, v);

    (8, 1)
}

// ld a,(0xff00+c)
#[allow(unused_variables)]
fn op_00e2(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(0xff00 + cpu.get_c() as u16);
    cpu.set_a(v);

    (8, 1)
}

// ld b,d8
#[allow(unused_variables)]
fn op_0006(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_b(v);

    (8, 2)
}

// ld c,d8
#[allow(unused_variables)]
fn op_000e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_c(v);

    (8, 2)
}

// ld d,d8
#[allow(unused_variables)]
fn op_0016(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_d(v);

    (8, 2)
}

// ld e,d8
#[allow(unused_variables)]
fn op_001e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_e(v);

    (8, 2)
}

// ld h,d8
#[allow(unused_variables)]
fn op_0006(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_h(v);

    (8, 2)
}

// ld l,d8
#[allow(unused_variables)]
fn op_0006(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_l(v);

    (8, 2)
}

// ld a,d8
#[allow(unused_variables)]
fn op_0006(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_a(v);

    (8, 2)
}

// ld (hl),d8
#[allow(unused_variables)]
fn op_0036(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    mmu.set8(cpu.get_hl(), v);

    (12, 2)
}

// ld (0xff00+a8),a
#[allow(unused_variables)]
fn op_00e0(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_a();
    mmu.set8(0xff00 + mmu.get8(cpu.get_pc().wrapping_add(arg)) as u16, v);

    (12, 2)
}

// ld a,(0xff00+a8)
#[allow(unused_variables)]
fn op_00f0(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(0xff00 + mmu.get8(cpu.get_pc().wrapping_add(arg)) as u16);
    cpu.set_a(v);

    (12, 2)
}

// ld bc,d16
#[allow(unused_variables)]
fn op_0001(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_bc(v);

    (12, 3)
}

// ld de,d16
#[allow(unused_variables)]
fn op_0011(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_de(v);

    (12, 3)
}

// ld hl,d16
#[allow(unused_variables)]
fn op_0021(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_hl(v);

    (12, 3)
}

// ld sp,d16
#[allow(unused_variables)]
fn op_0031(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_sp(v);

    (12, 3)
}

// ld (a16),sp
#[allow(unused_variables)]
fn op_0008(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_sp();
    mmu.set16(mmu.ge16(cpu.get_pc().wrapping_add(arg)), v);

    (20, 3)
}

// ld (a16),a
#[allow(unused_variables)]
fn op_00ea(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_sp();
    mmu.set8(mmu.get16(cpu.get_pc().wrapping_add(arg)), v);

    (16, 3)
}

// ld a,(a16)
#[allow(unused_variables)]
fn op_00fa(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(mmu.get16(cpu.get_pc().wrapping_add(arg)));
    cpu.set_a();

    (16, 3)
}

// ld sp,hl
#[allow(unused_variables)]
fn op_00f9(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_hl();
    cpu.set_sp(v);

    (8, 1)
}

// ldhl sp,r8
#[allow(unused_variables)]
fn op_00f9(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_sp();
    let q = mmuget.8(cpu_get_pc().wrapping_add(arg));
    let (v, h, c, z) = alu::add16e(p, q, false);
    cpu.set_hl(v);
    cpu.set_zf(false);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (12, 2)
}

// cp b
#[allow(unused_variables)]
fn op_00b8(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = cpu.get_b();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// cp c
#[allow(unused_variables)]
fn op_00b9(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = cpu.get_c();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// cp d
#[allow(unused_variables)]
fn op_00ba(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = cpu.get_d();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// cp e
#[allow(unused_variables)]
fn op_00bb(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = cpu.get_e();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// cp h
#[allow(unused_variables)]
fn op_00bc(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = cpu.get_h();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// cp l
#[allow(unused_variables)]
fn op_00bd(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = cpu.get_l();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// cp a
#[allow(unused_variables)]
fn op_00bf(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = cpu.get_a();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// cp (hl)
#[allow(unused_variables)]
fn op_00be(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_hl());
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

// cp d8
#[allow(unused_variables)]
fn op_00be(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

// jp a16
#[allow(unused_variables)]
fn op_00c3(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_pc(pc.wrapping_sub(3));
    //????
    (16, 3)
}

// jp nz,a16
#[allow(unused_variables)]
fn op_00c2(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = !cpu.get_zf();
    if flg{
        let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
        cpu.set_pc(pc);
        return (16, 0)
    }

    (12, 3)
}

// jp z,a16
#[allow(unused_variables)]
fn op_00ca(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = cpu.get_zf();
    if flg{
        let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
        cpu.set_pc(pc);
        return (16, 0)
    }

    (12, 3)
}

// jp nc,a16
#[allow(unused_variables)]
fn op_00d2(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = !cpu.get_cf();
    if flg{
        let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
        cpu.set_pc(pc);
        return (16, 0)
    }

    (12, 3)
}

// jp c,a16
#[allow(unused_variables)]
fn op_00c2(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = cpu.get_zf();
    if flg{
        let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
        cpu.set_pc(pc);
        return (16, 0)
    }

    (12, 3)
}

// jp hl
#[allow(unused_variables)]
fn op_00e9(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let pc = cpu.get_hl();
    cpu.set_pc(pc.wrapping_sub(1));

    (4, 1)
}

// jr r8
#[allow(unused_variables)]
fn op_0018(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let pc = cpu.get_pc().wrapping_add(alu::signed(p));
    cpu.set_pc(pc);

    (12, 2)
}

// jr z,r8
#[allow(unused_variables)]
fn op_0028(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flag = cpu.get_zf();
    if flg{
        let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
        let pc = cpu.getpc().wrapping_add(alu::signed(p));
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

// jr nz,r8
#[allow(unused_variables)]
fn op_0020(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flag = !cpu.get_zf();
    if flg{
        let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
        let pc = cpu.getpc().wrapping_add(alu::signed(p));
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

// jr c,r8
#[allow(unused_variables)]
fn op_0038(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flag = cpu.get_cf();
    if flg{
        let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
        let pc = cpu.getpc().wrapping_add(alu::signed(p));
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

// jr nc,r8
#[allow(unused_variables)]
fn op_0030(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flag = !cpu.get_cf();
    if flg{
        let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
        let pc = cpu.getpc().wrapping_add(alu::signed(p));
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

// push bc
#[allow(unused_variables)]
fn op_00c5(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_bc());

    (16, 1)
}

// push de
#[allow(unused_variables)]
fn op_00d5(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_de());

    (16, 1)
}

// push hl
#[allow(unused_variables)]
fn op_00e5(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_hl());

    (16, 1)
}

// push af
#[allow(unused_variables)]
fn op_00f5(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_af());

    (16, 1)
}

// pop bc
#[allow(unused_variables)]
fn op_00c1(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.pop(mmu);
    cpu.set_bc(v);

    (12, 1)
}

// pop de
#[allow(unused_variables)]
fn op_00d1(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.pop(mmu);
    cpu.set_de(v);

    (12, 1)
}

// pop hl
#[allow(unused_variables)]
fn op_00e1(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.pop(mmu);
    cpu.set_hl(v);

    (12, 1)
}

// pop af
#[allow(unused_variables)]
fn op_00f1(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.pop(mmu);
    cpu.set_af(v);

    (12, 1)
}

// call a16
#[allow(unused_variables)]
fn op_00cd(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(3));
    cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)).wrapping_sub(3));

    (24, 3)
}

// call nz,a16
#[allow(unused_variables)]
fn op_00c4(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = !cpu.get_zf();
    if flg{
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)));
        return (24, 0);
    }
    

    (12, 3)
}

// call z,a16
#[allow(unused_variables)]
fn op_00cc(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = cpu.get_zf();
    if flg{
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)));
        return (24, 0);
    }
    

    (12, 3)
}

// call nc,a16
#[allow(unused_variables)]
fn op_00d4(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = !cpu.get_cf();
    if flg{
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)));
        return (24, 0);
    }
    

    (12, 3)
}

// call c,a16
#[allow(unused_variables)]
fn op_00dc(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = cpu.get_cf();
    if flg{
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)));
        return (24, 0);
    }
    

    (12, 3)
}

// ret
#[allow(unused_variables)]
fn op_00c9(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let pc = cpu.pop(mmu).wrapping_sub(1);
    cpu.set_pc(pc);

    (16, 1)
}

// ret nz
#[allow(unused_variables)]
fn op_00c0(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = !cpu.get_zf();
    if flg {
        let pc = cpu.pop(mmu);
        cpu.set_pc(pc);
        return (20, 0);
    }

    (8, 1)
}

// ret z
#[allow(unused_variables)]
fn op_00c8(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = cpu.get_zf();
    if flg {
        let pc = cpu.pop(mmu);
        cpu.set_pc(pc);
        return (20, 0);
    }
    
    (8, 1)
}

// ret nc
#[allow(unused_variables)]
fn op_00d0(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = !cpu.get_cf();
    if flg {
        let pc = cpu.pop(mmu);
        cpu.set_pc(pc);
        return (20, 0);
    }
    
    (8, 1)
}

// ret c
#[allow(unused_variables)]
fn op_00d8(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = cpu.get_cf();
    if flg {
        let pc = cpu.pop(mmu);
        cpu.set_pc(pc);
        return (20, 0);
    }
    
    (8, 1)
}

// reti
#[allow(unused_variables)]
fn op_00c9(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let pc = cpu.pop(mmu).wrapping_sub(1);
    cpu.set_pc(pc);
    cpu.enable_interrupt();

    (16, 1)
}

// rst 0x00
#[allow(unused_variables)]
fn op_00c7(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x00u16.wrapping_sub(1));

    (16, 1)
}

// rst 0x08
#[allow(unused_variables)]
fn op_00cf(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x08u16.wrapping_sub(1));

    (16, 1)
}

// rst 0x10
#[allow(unused_variables)]
fn op_00d7(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x10u16.wrapping_sub(1));

    (16, 1)
}

// rst 0x18
#[allow(unused_variables)]
fn op_00df(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x18u16.wrapping_sub(1));

    (16, 1)
}

// rst 0x20
#[allow(unused_variables)]
fn op_00e7(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x20u16.wrapping_sub(1));

    (16, 1)
}

// rst 0x28
#[allow(unused_variables)]
fn op_00ef(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x28u16.wrapping_sub(1));

    (16, 1)
}

// rst 0x30
#[allow(unused_variables)]
fn op_00f7(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x30u16.wrapping_sub(1));

    (16, 1)
}

// rst 0x38
#[allow(unused_variables)]
fn op_00ff(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x38u16.wrapping_sub(1));

    (16, 1)
}

// inc b
#[allow(unused_variables)]
fn op_0004(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_b();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

// inc c
#[allow(unused_variables)]
fn op_000c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_c();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

// inc d
#[allow(unused_variables)]
fn op_0014(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_d();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

// inc e
#[allow(unused_variables)]
fn op_001c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_e();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

// inc h
#[allow(unused_variables)]
fn op_0024(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_h();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

// inc l
#[allow(unused_variables)]
fn op_002c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_l();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

// inc a
#[allow(unused_variables)]
fn op_003c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_a();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

// inc bc
#[allow(unused_variables)]
fn op_0003(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_bc().wrapping_add(1);
    cpu.set_bc(v);

    (8, 1)
}

// inc de
#[allow(unused_variables)]
fn op_0013(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_de().wrapping_add(1);
    cpu.set_de(v);

    (8, 1)
}

// inc hl
#[allow(unused_variables)]
fn op_0023(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_hl().wrapping_add(1);
    cpu.set_hl(v);

    (8, 1)
}

// inc sp
#[allow(unused_variables)]
fn op_0033(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_sp().wrapping_add(1);
    cpu.set_sp(v);

    (8, 1)
}

// dec b
#[allow(unused_variables)]
fn op_0005(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_b();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

// dec c
#[allow(unused_variables)]
fn op_000d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_c();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

// dec d
#[allow(unused_variables)]
fn op_0015(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_d();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

// dec e
#[allow(unused_variables)]
fn op_001d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_e();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

// dec h
#[allow(unused_variables)]
fn op_0025(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_h();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

// dec l
#[allow(unused_variables)]
fn op_002d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_l();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

// dec a
#[allow(unused_variables)]
fn op_003d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_a();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

// inc (hl)
#[allow(unused_variables)]
fn op_0034(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::add8(v, 1, false);
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (12, 1)
}

// dec (hl)
#[allow(unused_variables)]
fn op_0035(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::sub8(v, 1, false);
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (12, 1)
}

// dec bc
#[allow(unused_variables)]
fn op_000b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_bc().wrapping_sub(1);
    cpu.set_bc(v);

    (8, 1)
}

// dec de
#[allow(unused_variables)]
fn op_001b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_de().wrapping_sub(1);
    cpu.set_de(v);

    (8, 1)
}

// dec hl
#[allow(unused_variables)]
fn op_002b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_hl().wrapping_sub(1);
    cpu.set_hl(v);

    (8, 1)
}

// dec sp
#[allow(unused_variables)]
fn op_003b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_sp().wrapping_sub(1);
    cpu.set_sp(v);

    (8, 1)
}

// add a,b
#[allow(unused_variables)]
fn op_0080(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_b();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// add a,c
#[allow(unused_variables)]
fn op_0081(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_c();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// add a,d
#[allow(unused_variables)]
fn op_0082(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_d();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// add a,e
#[allow(unused_variables)]
fn op_0083(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_e();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// add a,h
#[allow(unused_variables)]
fn op_0084(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_h();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// add a,l
#[allow(unused_variables)]
fn op_0085(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_l();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// add a,a
#[allow(unused_variables)]
fn op_0087(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_a();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// add a,d8
#[allow(unused_variables)]
fn op_00c6(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

// add a,(hl)
#[allow(unused_variables)]
fn op_0080(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

// add hl,bc
#[allow(unused_variables)]
fn op_0009(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_hl();
    let q = cpu.get_bc();
    let (v, h, c, z) = alu::add16(p, q, false);
    cpu.set_hl(v);
    
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

// add hl,de
#[allow(unused_variables)]
fn op_0019(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_hl();
    let q = cpu.get_de();
    let (v, h, c, z) = alu::add16(p, q, false);
    cpu.set_hl(v);
    
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

// add hl,hl
#[allow(unused_variables)]
fn op_0029(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_hl();
    let q = cpu.get_hl();
    let (v, h, c, z) = alu::add16(p, q, false);
    cpu.set_hl(v);
    
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

// add hl,sp
#[allow(unused_variables)]
fn op_0039(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_hl();
    let q = cpu.get_sp();
    let (v, h, c, z) = alu::add16(p, q, false);
    cpu.set_hl(v);
    
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

// add sp,r8
#[allow(unused_variables)]
fn op_0009(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_sp();
    let q = cpu.get8(cpu.get_pc().wrapping_add(arg));
    let (v, h, c, z) = alu::add16e(p, q, false);
    cpu.set_sp(v);
    
    cpu.set_zf(false)
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (16, 2)
}

// adc a,b
#[allow(unused_variables)]
fn op_0088(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_b();
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// adc a,c
#[allow(unused_variables)]
fn op_0089(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_c();
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// adc a,d
#[allow(unused_variables)]
fn op_008a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_d();
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// adc a,e
#[allow(unused_variables)]
fn op_008b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_e();
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// adc a,h
#[allow(unused_variables)]
fn op_008c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get.h();
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// adc a,l
#[allow(unused_variables)]
fn op_008d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_l();
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// adc a,a
#[allow(unused_variables)]
fn op_008f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_a();
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// adc a,d8
#[allow(unused_variables)]
fn op_00ce(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

// adc a,(hl)
#[allow(unused_variables)]
fn op_008e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

// sub b
#[allow(unused_variables)]
fn op_0090(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_b();
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// sub c
#[allow(unused_variables)]
fn op_0091(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_c();
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// sub d
#[allow(unused_variables)]
fn op_0092(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_d();
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// sub e
#[allow(unused_variables)]
fn op_0093(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_e();
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// sub h
#[allow(unused_variables)]
fn op_0094(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_h();
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// sub l
#[allow(unused_variables)]
fn op_0095(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_l();
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// sub a
#[allow(unused_variables)]
fn op_0097(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_a();
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// sub d8
#[allow(unused_variables)]
fn op_00d6(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

// sub (hl)
#[allow(unused_variables)]
fn op_0096(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

// sbc a,b
#[allow(unused_variables)]
fn op_0098(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_b();
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// sbc a,c
#[allow(unused_variables)]
fn op_0099(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_c();
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// sbc a,d
#[allow(unused_variables)]
fn op_009a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_d();
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// sbc a,e
#[allow(unused_variables)]
fn op_009b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_e();
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// sbc a,h
#[allow(unused_variables)]
fn op_009c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_h();
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// sbc a,l
#[allow(unused_variables)]
fn op_009d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_l();
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// sbc a,a
#[allow(unused_variables)]
fn op_009f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_a();
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// sbc a,d8
#[allow(unused_variables)]
fn op_00de(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

// sbc a,(hl)
#[allow(unused_variables)]
fn op_009e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

// adc a,d8
#[allow(unused_variables)]
fn op_00ce(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

// and b
#[allow(unused_variables)]
fn op_00a0(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_b());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

// and c
#[allow(unused_variables)]
fn op_00a1(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_c());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

// and d
#[allow(unused_variables)]
fn op_00a2(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_d());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

// and e
#[allow(unused_variables)]
fn op_00a3(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_e());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

// and h
#[allow(unused_variables)]
fn op_00a4(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_h());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

// and l
#[allow(unused_variables)]
fn op_00a5(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_l());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

// and a
#[allow(unused_variables)]
fn op_00a7(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_a());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

// and d8
#[allow(unused_variables)]
fn op_00a0(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & mmu.get8(cpu.get_pc().wrapping_add(arg)));
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (8, 2)
}

// and (hl)
#[allow(unused_variables)]
fn op_00a6(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & mmu.get8(cpu.get_hl()));
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

// xor b
#[allow(unused_variables)]
fn op_00a8(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu_get_a(cpu.get_a() ^ cpu.get_b());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// xor c
#[allow(unused_variables)]
fn op_00a9(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu_get_a(cpu.get_a() ^ cpu.get_c());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// xor d
#[allow(unused_variables)]
fn op_00aa(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu_get_a(cpu.get_a() ^ cpu.get_d());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// xor e
#[allow(unused_variables)]
fn op_00ab(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu_get_a(cpu.get_a() ^ cpu.get_e());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// xor h
#[allow(unused_variables)]
fn op_00ac(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu_get_a(cpu.get_a() ^ cpu.get_h());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// xor l
#[allow(unused_variables)]
fn op_00ad(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu_get_a(cpu.get_a() ^ cpu.get_l());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// xor a
#[allow(unused_variables)]
fn op_00af(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu_get_a(cpu.get_a() ^ cpu.get_a());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// xor d8
#[allow(unused_variables)]
fn op_00ee(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ mmu.get8(cpu.get_pc().wrapping_add(arg)));
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

// xor (hl)
#[allow(unused_variables)]
fn op_00ae(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ mmu.get8(cpu.get_hl()));
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// or b
#[allow(unused_variables)]
fn op_00b0(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu_get_a(cpu.get_a() | cpu.get_b());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// or c
#[allow(unused_variables)]
fn op_00b1(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu_get_a(cpu.get_a() | cpu.get_c());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// or d
#[allow(unused_variables)]
fn op_00b2(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu_get_a(cpu.get_a() | cpu.get_d());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// or e
#[allow(unused_variables)]
fn op_00b3(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu_get_a(cpu.get_a() | cpu.get_e());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// or h
#[allow(unused_variables)]
fn op_00b4(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu_get_a(cpu.get_a() | cpu.get_h());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// or l
#[allow(unused_variables)]
fn op_00b5(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu_get_a(cpu.get_a() | cpu.get_l());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// or a
#[allow(unused_variables)]
fn op_00b7(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu_get_a(cpu.get_a() | cpu.get_a());
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// or d8
#[allow(unused_variables)]
fn op_00f6(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | mmu.get8(cpu.get_pc().wrapping_add(arg)));
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

// or (hl)
#[allow(unused_variables)]
fn op_00b6(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | mmu.get8(cpu.get_hl()));
    let z = cpu.get_a() == 0;

    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

// cpl
#[allow(unused_variables)]
fn op_00b6(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ 0xff);

    cpu.set_nf(true);
    cpu.set_hf(true);

    (4, 1)
}

// forward
// daa
#[allow(unused_variables)]
fn op_00b6(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let mut adj = 0;

    let v = cpu.get_a() as usize;

    if cpu.get_hf() || (!cpu.get_nf() && (v & 0xf) > 9) {
        adj |= 0x6;
    }

    let c = if cpu.get_cf() || (!cpu.get_nf() && v > 0x99) {
        adj |= 0x60;
        true
    } else {
        false
    };

    let v = if cpu.get_nf() { v - adj } else { v + adj };
    let v = (v & 0xff) as u8;
    let z = v == 0;

    cpu.set_a(v);
    cpu.set_zf(z);

    cpu.set_hf(false);
    cpu.set_cf(c);

    (4, 1)
}

// scf
#[allow(unused_variables)]
fn op_0037(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    cpu.set_cf(true);

    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(true);

    (4, 1)
}

// ccf
#[allow(unused_variables)]
fn op_0037(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let c = !cpu.get_cf();

    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (4, 1)
}

// rlc b
#[allow(unused_variables)]
fn op_cb00(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rlc c
#[allow(unused_variables)]
fn op_cb01(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rlc d
#[allow(unused_variables)]
fn op_cb02(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rlc e
#[allow(unused_variables)]
fn op_cb03(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rlc h
#[allow(unused_variables)]
fn op_cb04(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rlc l
#[allow(unused_variables)]
fn op_cb05(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rlc (hl)
#[allow(unused_variables)]
fn op_cb06(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (16, 2)
}

// rlc a
#[allow(unused_variables)]
fn op_cb07(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rrc b
#[allow(unused_variables)]
fn op_cb08(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let c = v & 0x80 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rrc c
#[allow(unused_variables)]
fn op_cb09(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let c = v & 0x80 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rrc d
#[allow(unused_variables)]
fn op_cb0a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let c = v & 0x80 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rrc e
#[allow(unused_variables)]
fn op_cb0b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let c = v & 0x80 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rrc h
#[allow(unused_variables)]
fn op_cb0c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let c = v & 0x80 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rrc l
#[allow(unused_variables)]
fn op_cb0d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let c = v & 0x80 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rrc (hl)
#[allow(unused_variables)]
fn op_cb0e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let c = v & 0x80 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (16, 2)
}

// rrc a
#[allow(unused_variables)]
fn op_cb0f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 0x80 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rl b
#[allow(unused_variables)]
fn op_cb10(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rl c
#[allow(unused_variables)]
fn op_cb11(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rl d
#[allow(unused_variables)]
fn op_cb12(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rl e
#[allow(unused_variables)]
fn op_cb13(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rl h
#[allow(unused_variables)]
fn op_cb14(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rl l
#[allow(unused_variables)]
fn op_cb15(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rl (hl)
#[allow(unused_variables)]
fn op_cb16(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (16, 2)
}

// rl a
#[allow(unused_variables)]
fn op_cb17(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rr b
#[allow(unused_variables)]
fn op_cb18(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rr c
#[allow(unused_variables)]
fn op_cb19(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let c = v & 1 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rr d
#[allow(unused_variables)]
fn op_cb1a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rr e
#[allow(unused_variables)]
fn op_cb1b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rr h
#[allow(unused_variables)]
fn op_cb1c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rr l
#[allow(unused_variables)]
fn op_cb1d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// rr (hl)
#[allow(unused_variables)]
fn op_cb1e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let c = v & 1 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (16, 2)
}

// rr a
#[allow(unused_variables)]
fn op_cb1f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// sla b
#[allow(unused_variables)]
fn op_cb20(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// sla c
#[allow(unused_variables)]
fn op_cb21(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// sla d
#[allow(unused_variables)]
fn op_cb22(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// sla e
#[allow(unused_variables)]
fn op_cb23(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// sla h
#[allow(unused_variables)]
fn op_cb24(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// sla l
#[allow(unused_variables)]
fn op_cb25(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// sla (hl)
#[allow(unused_variables)]
fn op_cb16(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (16, 2)
}

// sla a
#[allow(unused_variables)]
fn op_cb27(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// sra b
#[allow(unused_variables)]
fn op_cb28(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// sra c
#[allow(unused_variables)]
fn op_cb29(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// sra d
#[allow(unused_variables)]
fn op_cb2a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// sra e
#[allow(unused_variables)]
fn op_cb2b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// sra h
#[allow(unused_variables)]
fn op_cb2c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// sra l
#[allow(unused_variables)]
fn op_cb2d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// sra (hl)
#[allow(unused_variables)]
fn op_cb2e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (16, 2)
}

// sra a
#[allow(unused_variables)]
fn op_cb2f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// swap b
#[allow(unused_variables)]
fn op_cb30(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let v = v.rotate_left(4);
    cpu.set_b(v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

// swap c
#[allow(unused_variables)]
fn op_cb31(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let v = v.rotate_left(4);
    cpu.set_c(v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

// swap d
#[allow(unused_variables)]
fn op_cb32(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let v = v.rotate_left(4);
    cpu.set_d(v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

// swap e
#[allow(unused_variables)]
fn op_cb33(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let v = v.rotate_left(4);
    cpu.set_e(v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

// swap h
#[allow(unused_variables)]
fn op_cb34(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let v = v.rotate_left(4);
    cpu.set_h(v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

// swap l
#[allow(unused_variables)]
fn op_cb35(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let v = v.rotate_left(4);
    cpu.set_l(v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

// swap hl
#[allow(unused_variables)]
fn op_cb36(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let v = v.rotate_left(4);
    mmu.set8(cpu.get_hl(), v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (16, 2)
}

// swap a
#[allow(unused_variables)]
fn op_cb37(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let v = v.rotate_left(4);
    cpu.set_a(v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

// srl b
#[allow(unused_variables)]
fn op_cb38(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// srl c
#[allow(unused_variables)]
fn op_cb39(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// srl d
#[allow(unused_variables)]
fn op_cb3a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// srl e
#[allow(unused_variables)]
fn op_cb3b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// srl h
#[allow(unused_variables)]
fn op_cb3c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// srl l
#[allow(unused_variables)]
fn op_cb3d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// srl (hl)
#[allow(unused_variables)]
fn op_cb3e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (16, 2)
}

// srl a
#[allow(unused_variables)]
fn op_cb3f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

// bit 0,b
#[allow(unused_variables)]
fn op_cb40(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 0,c
#[allow(unused_variables)]
fn op_cb41(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 0,d
#[allow(unused_variables)]
fn op_cb42(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 0,e
#[allow(unused_variables)]
fn op_cb43(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 0,h
#[allow(unused_variables)]
fn op_cb44(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 0,l
#[allow(unused_variables)]
fn op_cb45(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 0,(hl)
#[allow(unused_variables)]
fn op_cb46(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (12, 2)
}

// bit 0,a
#[allow(unused_variables)]
fn op_cb47(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 1,b
#[allow(unused_variables)]
fn op_cb48(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 1,c
#[allow(unused_variables)]
fn op_cb49(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 1,d
#[allow(unused_variables)]
fn op_cb4a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 1,e
#[allow(unused_variables)]
fn op_cb4b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 1,h
#[allow(unused_variables)]
fn op_cb4c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 1,l
#[allow(unused_variables)]
fn op_cb4d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 1,(hl)
#[allow(unused_variables)]
fn op_cb4e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (12, 2)
}

// bit 1,a
#[allow(unused_variables)]
fn op_cb4f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 2,b
#[allow(unused_variables)]
fn op_cb50(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 2,c
#[allow(unused_variables)]
fn op_cb51(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 2,d
#[allow(unused_variables)]
fn op_cb52(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 2,e
#[allow(unused_variables)]
fn op_cb53(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 2,h
#[allow(unused_variables)]
fn op_cb54(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 2,l
#[allow(unused_variables)]
fn op_cb55(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 2,(hl)
#[allow(unused_variables)]
fn op_cb56(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (12, 2)
}

// bit 2,a
#[allow(unused_variables)]
fn op_cb57(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 3,b
#[allow(unused_variables)]
fn op_cb58(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 3,c
#[allow(unused_variables)]
fn op_cb59(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 3,d
#[allow(unused_variables)]
fn op_cb5a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 3,e
#[allow(unused_variables)]
fn op_cb5b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 3,h
#[allow(unused_variables)]
fn op_cb5c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 3,l
#[allow(unused_variables)]
fn op_cb5d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 3,(hl)
#[allow(unused_variables)]
fn op_cb5e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (12, 2)
}

// bit 3,a
#[allow(unused_variables)]
fn op_cb5f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 4,b
#[allow(unused_variables)]
fn op_cb60(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 4,c
#[allow(unused_variables)]
fn op_cb61(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 4,d
#[allow(unused_variables)]
fn op_cb62(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 4,e
#[allow(unused_variables)]
fn op_cb63(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 4,h
#[allow(unused_variables)]
fn op_cb64(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 4,l
#[allow(unused_variables)]
fn op_cb65(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 4,(hl)
#[allow(unused_variables)]
fn op_cb66(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (12, 2)
}

// bit 4,a
#[allow(unused_variables)]
fn op_cb67(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 5,b
#[allow(unused_variables)]
fn op_cb68(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 5,c
#[allow(unused_variables)]
fn op_cb69(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 5,d
#[allow(unused_variables)]
fn op_cb6a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 5,e
#[allow(unused_variables)]
fn op_cb6b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 5,h
#[allow(unused_variables)]
fn op_cb6c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 5,l
#[allow(unused_variables)]
fn op_cb6d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 5,(hl)
#[allow(unused_variables)]
fn op_cb6e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (12, 2)
}

// bit 5,a
#[allow(unused_variables)]
fn op_cb6f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 6,b
#[allow(unused_variables)]
fn op_cb70(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 6,c
#[allow(unused_variables)]
fn op_cb71(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 6,d
#[allow(unused_variables)]
fn op_cb72(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 6,e
#[allow(unused_variables)]
fn op_cb73(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 6,h
#[allow(unused_variables)]
fn op_cb74(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 6,l
#[allow(unused_variables)]
fn op_cb75(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 6,(hl)
#[allow(unused_variables)]
fn op_cb76(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (12, 2)
}

// bit 6,a
#[allow(unused_variables)]
fn op_cb77(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 7,b
#[allow(unused_variables)]
fn op_cb78(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 7,c
#[allow(unused_variables)]
fn op_cb79(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 7,d
#[allow(unused_variables)]
fn op_cb7a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 7,e
#[allow(unused_variables)]
fn op_cb7b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 7,h
#[allow(unused_variables)]
fn op_cb7c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 7,l
#[allow(unused_variables)]
fn op_cb7d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

// bit 7,(hl)
#[allow(unused_variables)]
fn op_cb7e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (12, 2)
}

// bit 7,a
#[allow(unused_variables)]
fn op_cb7f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// res 0,b
#[allow(unused_variables)]
fn op_cb80(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 0,c
#[allow(unused_variables)]
fn op_cb81(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 0,d
#[allow(unused_variables)]
fn op_cb82(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 0,e
#[allow(unused_variables)]
fn op_cb83(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 0,h
#[allow(unused_variables)]
fn op_cb84(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 0,l
#[allow(unused_variables)]
fn op_cb85(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 0,(hl)
#[allow(unused_variables)]
fn op_cb86(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 0,a
#[allow(unused_variables)]
fn op_cb87(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 1,b
#[allow(unused_variables)]
fn op_cb88(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 1,c
#[allow(unused_variables)]
fn op_cb89(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 1,d
#[allow(unused_variables)]
fn op_cb8a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 1,e
#[allow(unused_variables)]
fn op_cb8b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 1,h
#[allow(unused_variables)]
fn op_cb8c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 1,l
#[allow(unused_variables)]
fn op_cb8d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 1,(hl)
#[allow(unused_variables)]
fn op_cb8e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 1,a
#[allow(unused_variables)]
fn op_cb8f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 2,b
#[allow(unused_variables)]
fn op_cb90(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 2,c
#[allow(unused_variables)]
fn op_cb91(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 2,d
#[allow(unused_variables)]
fn op_cb92(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 2,e
#[allow(unused_variables)]
fn op_cb93(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 2,h
#[allow(unused_variables)]
fn op_cb94(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 2,l
#[allow(unused_variables)]
fn op_cb95(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 2,(hl)
#[allow(unused_variables)]
fn op_cb96(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 2,a
#[allow(unused_variables)]
fn op_cb97(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 3,b
#[allow(unused_variables)]
fn op_cb98(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 3,c
#[allow(unused_variables)]
fn op_cb99(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 3,d
#[allow(unused_variables)]
fn op_cb9a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 3,e
#[allow(unused_variables)]
fn op_cb9b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 3,h
#[allow(unused_variables)]
fn op_cb9c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 3,l
#[allow(unused_variables)]
fn op_cb9d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 3,(hl)
#[allow(unused_variables)]
fn op_cb9e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 3,a
#[allow(unused_variables)]
fn op_cb9f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 4,b
#[allow(unused_variables)]
fn op_cba0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 4,c
#[allow(unused_variables)]
fn op_cba1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 4,d
#[allow(unused_variables)]
fn op_cba2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 4,e
#[allow(unused_variables)]
fn op_cba3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 4,h
#[allow(unused_variables)]
fn op_cba4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 4,l
#[allow(unused_variables)]
fn op_cba5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 4,(hl)
#[allow(unused_variables)]
fn op_cba6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 4,a
#[allow(unused_variables)]
fn op_cba7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 5,b
#[allow(unused_variables)]
fn op_cba8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 5,c
#[allow(unused_variables)]
fn op_cba9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 5,d
#[allow(unused_variables)]
fn op_cbaa(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 5,e
#[allow(unused_variables)]
fn op_cbab(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 5,h
#[allow(unused_variables)]
fn op_cbac(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 5,l
#[allow(unused_variables)]
fn op_cbad(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 5,(hl)
#[allow(unused_variables)]
fn op_cbae(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 5,a
#[allow(unused_variables)]
fn op_cbaf(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 6,b
#[allow(unused_variables)]
fn op_cbb0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 6,c
#[allow(unused_variables)]
fn op_cbb1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 6,d
#[allow(unused_variables)]
fn op_cbb2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 6,e
#[allow(unused_variables)]
fn op_cbb3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 6,h
#[allow(unused_variables)]
fn op_cbb4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 6,l
#[allow(unused_variables)]
fn op_cbb5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 6,(hl)
#[allow(unused_variables)]
fn op_cbb6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 6,a
#[allow(unused_variables)]
fn op_cbb7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 7,b
#[allow(unused_variables)]
fn op_cbb8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 7,c
#[allow(unused_variables)]
fn op_cbb9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 7,d
#[allow(unused_variables)]
fn op_cbba(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 7,e
#[allow(unused_variables)]
fn op_cbbb(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 7,h
#[allow(unused_variables)]
fn op_cbbc(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 7,l
#[allow(unused_variables)]
fn op_cbbd(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 7,(hl)
#[allow(unused_variables)]
fn op_cbbe(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 7,a
#[allow(unused_variables)]
fn op_cbbf(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// set 0,b
#[allow(unused_variables)]
fn op_cbc0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 0,c
#[allow(unused_variables)]
fn op_cbc1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 0,d
#[allow(unused_variables)]
fn op_cbc2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 0,e
#[allow(unused_variables)]
fn op_cbc3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 0,h
#[allow(unused_variables)]
fn op_cbc4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 0,l
#[allow(unused_variables)]
fn op_cbc5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 0,(hl)
#[allow(unused_variables)]
fn op_cbc6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 0,a
#[allow(unused_variables)]
fn op_cbc7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 1,b
#[allow(unused_variables)]
fn op_cbc8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 1,c
#[allow(unused_variables)]
fn op_cbc9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 1,d
#[allow(unused_variables)]
fn op_cbca(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 1,e
#[allow(unused_variables)]
fn op_cbcb(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 1,h
#[allow(unused_variables)]
fn op_cbcc(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 1,l
#[allow(unused_variables)]
fn op_cbcd(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 1,(hl)
#[allow(unused_variables)]
fn op_cbce(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 1,a
#[allow(unused_variables)]
fn op_cbcf(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 2,b
#[allow(unused_variables)]
fn op_cbd0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 2,c
#[allow(unused_variables)]
fn op_cbd1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 2,d
#[allow(unused_variables)]
fn op_cbd2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 2,e
#[allow(unused_variables)]
fn op_cbd3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 2,h
#[allow(unused_variables)]
fn op_cbd4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 2,l
#[allow(unused_variables)]
fn op_cbd5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 2,(hl)
#[allow(unused_variables)]
fn op_cbd6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 2,a
#[allow(unused_variables)]
fn op_cbd7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 3,b
#[allow(unused_variables)]
fn op_cbd8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 3,c
#[allow(unused_variables)]
fn op_cbd9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 3,d
#[allow(unused_variables)]
fn op_cbda(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 3,e
#[allow(unused_variables)]
fn op_cbdb(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 3,h
#[allow(unused_variables)]
fn op_cbdc(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 3,l
#[allow(unused_variables)]
fn op_cbdd(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 3,(hl)
#[allow(unused_variables)]
fn op_cbde(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 3,a
#[allow(unused_variables)]
fn op_cbdf(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 4,b
#[allow(unused_variables)]
fn op_cbe0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 4,c
#[allow(unused_variables)]
fn op_cbe1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 4,d
#[allow(unused_variables)]
fn op_cbe2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 4,e
#[allow(unused_variables)]
fn op_cbe3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 4,h
#[allow(unused_variables)]
fn op_cbe4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 4,l
#[allow(unused_variables)]
fn op_cbe5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 4,(hl)
#[allow(unused_variables)]
fn op_cbe6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 4,a
#[allow(unused_variables)]
fn op_cbe7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 5,b
#[allow(unused_variables)]
fn op_cbe8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 5,c
#[allow(unused_variables)]
fn op_cbe9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 5,d
#[allow(unused_variables)]
fn op_cbea(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 5,e
#[allow(unused_variables)]
fn op_cbeb(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 5,h
#[allow(unused_variables)]
fn op_cbec(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 5,l
#[allow(unused_variables)]
fn op_cbed(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 5,(hl)
#[allow(unused_variables)]
fn op_cbee(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 5,a
#[allow(unused_variables)]
fn op_cbef(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 6,b
#[allow(unused_variables)]
fn op_cbf0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 6,c
#[allow(unused_variables)]
fn op_cbf1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 6,d
#[allow(unused_variables)]
fn op_cbf2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 6,e
#[allow(unused_variables)]
fn op_cbf3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 6,h
#[allow(unused_variables)]
fn op_cbf4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 6,l
#[allow(unused_variables)]
fn op_cbf5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 6,(hl)
#[allow(unused_variables)]
fn op_cbf6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 6,a
#[allow(unused_variables)]
fn op_cbf7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 7,b
#[allow(unused_variables)]
fn op_cbf8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 7,c
#[allow(unused_variables)]
fn op_cbf9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 7,d
#[allow(unused_variables)]
fn op_cbfa(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 7,e
#[allow(unused_variables)]
fn op_cbfb(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 7,h
#[allow(unused_variables)]
fn op_cbfc(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 7,l
#[allow(unused_variables)]
fn op_cbfd(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 7,(hl)
#[allow(unused_variables)]
fn op_cbfe(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 7,a
#[allow(unused_variables)]
fn op_cbff(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// rlca
#[allow(unused_variables)]
fn op_0007(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(false);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (4, 1)
}

/// rrca
#[allow(unused_variables)]
fn op_000f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 1 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(false);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (4, 1)
}

/// rla
#[allow(unused_variables)]
fn op_0017(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    cpu.set_a(v);
    cpu.set_zf(false);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (4, 1)
}

/// rra
#[allow(unused_variables)]
fn op_001f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    cpu.set_a(v);
    cpu.set_zf(false);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (4, 1)
}

/// halt
#[allow(unused_variables)]
fn op_0076(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.halt();

    (4, 1)
}

/// stop 0
#[allow(unused_variables)]
fn op_0010(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.stop();

    (4, 2)
}

/// di
#[allow(unused_variables)]
fn op_00f3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.disable_interrupt();

    (4, 1)
}

/// ei
#[allow(unused_variables)]
fn op_00fb(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.enable_interrupt();

    (4, 1)
}