use super::periph;

pub fn init_hw() {
    let rcc = periph!(RCC);
    let pa = periph!(GPIOA);
    rcc.apb2enr.modify(|_, w| w.iopaen().enabled());

    pa.crl.modify(|_, w|
        w.cnf0().push_pull().mode0().output2()
         .cnf1().push_pull().mode1().output2()
         .cnf5().push_pull().mode5().output2()
         .cnf6().push_pull().mode6().output2()
         .cnf7().push_pull().mode7().output2()
    );

    pa.crh.modify(|_, w| w.cnf8().push_pull().mode8().output2());
}

pub fn task_5ms() {
    let pa = periph!(GPIOA);
    if pa.odr.read().odr0().bit_is_set() {
        pa.bsrr.write(|w| w.br0().reset());
    } else {
        pa.bsrr.write(|w| w.bs0().set());
    }
}

pub fn task_10ms() {
    let pa = periph!(GPIOA);
    if pa.odr.read().odr1().bit_is_set() {
        pa.bsrr.write(|w| w.br1().reset());
    } else {
        pa.bsrr.write(|w| w.bs1().set());
    }
}

pub fn task_20ms() {
    let pa = periph!(GPIOA);
    if pa.odr.read().odr5().bit_is_set() {
        pa.bsrr.write(|w| w.br5().reset());
    } else {
        pa.bsrr.write(|w| w.bs5().set());
    }
}

pub fn task_50ms() {
    let pa = periph!(GPIOA);
    if pa.odr.read().odr6().bit_is_set() {
        pa.bsrr.write(|w| w.br6().reset());
    } else {
        pa.bsrr.write(|w| w.bs6().set());
    }
}

pub fn task_100ms() {
    let pa = periph!(GPIOA);
    if pa.odr.read().odr7().bit_is_set() {
        pa.bsrr.write(|w| w.br7().reset());
    } else {
        pa.bsrr.write(|w| w.bs7().set());
    }
}

pub fn task_1000ms() {
    let pa = periph!(GPIOA);
    if pa.odr.read().odr8().bit_is_set() {
        pa.bsrr.write(|w| w.br8().reset());
    } else {
        pa.bsrr.write(|w| w.bs8().set());
    }
}