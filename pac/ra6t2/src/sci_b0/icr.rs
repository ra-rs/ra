#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IICDL` reader - SDA Delay Output Select"]
pub type IICDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IICDL` writer - SDA Delay Output Select"]
pub type IICDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICR_SPEC, u8, u8, 5, O>;
#[doc = "Field `IICINTM` reader - IIC Interrupt Mode Select"]
pub type IICINTM_R = crate::BitReader<IICINTM_A>;
#[doc = "IIC Interrupt Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICINTM_A {
    #[doc = "0: Use ACK/NACK interrupts."]
    _0 = 0,
    #[doc = "1: Use reception and transmission interrupts"]
    _1 = 1,
}
impl From<IICINTM_A> for bool {
    #[inline(always)]
    fn from(variant: IICINTM_A) -> Self {
        variant as u8 != 0
    }
}
impl IICINTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICINTM_A {
        match self.bits {
            false => IICINTM_A::_0,
            true => IICINTM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICINTM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICINTM_A::_1
    }
}
#[doc = "Field `IICINTM` writer - IIC Interrupt Mode Select"]
pub type IICINTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, IICINTM_A, O>;
impl<'a, const O: u8> IICINTM_W<'a, O> {
    #[doc = "Use ACK/NACK interrupts."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICINTM_A::_0)
    }
    #[doc = "Use reception and transmission interrupts"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICINTM_A::_1)
    }
}
#[doc = "Field `IICCSC` reader - Clock Synchronization"]
pub type IICCSC_R = crate::BitReader<IICCSC_A>;
#[doc = "Clock Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICCSC_A {
    #[doc = "0: No synchronization with the clock signal"]
    _0 = 0,
    #[doc = "1: Synchronization with the clock signal"]
    _1 = 1,
}
impl From<IICCSC_A> for bool {
    #[inline(always)]
    fn from(variant: IICCSC_A) -> Self {
        variant as u8 != 0
    }
}
impl IICCSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICCSC_A {
        match self.bits {
            false => IICCSC_A::_0,
            true => IICCSC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICCSC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICCSC_A::_1
    }
}
#[doc = "Field `IICCSC` writer - Clock Synchronization"]
pub type IICCSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, IICCSC_A, O>;
impl<'a, const O: u8> IICCSC_W<'a, O> {
    #[doc = "No synchronization with the clock signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICCSC_A::_0)
    }
    #[doc = "Synchronization with the clock signal"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICCSC_A::_1)
    }
}
#[doc = "Field `IICACKT` reader - ACK Transmission Data"]
pub type IICACKT_R = crate::BitReader<IICACKT_A>;
#[doc = "ACK Transmission Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICACKT_A {
    #[doc = "0: ACK transmission"]
    _0 = 0,
    #[doc = "1: NACK transmission and reception of ACK/NACK"]
    _1 = 1,
}
impl From<IICACKT_A> for bool {
    #[inline(always)]
    fn from(variant: IICACKT_A) -> Self {
        variant as u8 != 0
    }
}
impl IICACKT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICACKT_A {
        match self.bits {
            false => IICACKT_A::_0,
            true => IICACKT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICACKT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICACKT_A::_1
    }
}
#[doc = "Field `IICACKT` writer - ACK Transmission Data"]
pub type IICACKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, IICACKT_A, O>;
impl<'a, const O: u8> IICACKT_W<'a, O> {
    #[doc = "ACK transmission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICACKT_A::_0)
    }
    #[doc = "NACK transmission and reception of ACK/NACK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICACKT_A::_1)
    }
}
#[doc = "Field `IICSTAREQ` reader - Start Condition Generation"]
pub type IICSTAREQ_R = crate::BitReader<IICSTAREQ_A>;
#[doc = "Start Condition Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICSTAREQ_A {
    #[doc = "0: A start condition is not generated"]
    _0 = 0,
    #[doc = "1: A start condition is generated."]
    _1 = 1,
}
impl From<IICSTAREQ_A> for bool {
    #[inline(always)]
    fn from(variant: IICSTAREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IICSTAREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICSTAREQ_A {
        match self.bits {
            false => IICSTAREQ_A::_0,
            true => IICSTAREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICSTAREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICSTAREQ_A::_1
    }
}
#[doc = "Field `IICSTAREQ` writer - Start Condition Generation"]
pub type IICSTAREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, IICSTAREQ_A, O>;
impl<'a, const O: u8> IICSTAREQ_W<'a, O> {
    #[doc = "A start condition is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICSTAREQ_A::_0)
    }
    #[doc = "A start condition is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICSTAREQ_A::_1)
    }
}
#[doc = "Field `IICRSTAREQ` reader - Restart Condition Generation"]
pub type IICRSTAREQ_R = crate::BitReader<IICRSTAREQ_A>;
#[doc = "Restart Condition Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICRSTAREQ_A {
    #[doc = "0: A restart condition is not generated."]
    _0 = 0,
    #[doc = "1: A restart condition is generated."]
    _1 = 1,
}
impl From<IICRSTAREQ_A> for bool {
    #[inline(always)]
    fn from(variant: IICRSTAREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IICRSTAREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICRSTAREQ_A {
        match self.bits {
            false => IICRSTAREQ_A::_0,
            true => IICRSTAREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICRSTAREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICRSTAREQ_A::_1
    }
}
#[doc = "Field `IICRSTAREQ` writer - Restart Condition Generation"]
pub type IICRSTAREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, IICRSTAREQ_A, O>;
impl<'a, const O: u8> IICRSTAREQ_W<'a, O> {
    #[doc = "A restart condition is not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICRSTAREQ_A::_0)
    }
    #[doc = "A restart condition is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICRSTAREQ_A::_1)
    }
}
#[doc = "Field `IICSTPREQ` reader - Stop Condition Generation"]
pub type IICSTPREQ_R = crate::BitReader<IICSTPREQ_A>;
#[doc = "Stop Condition Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICSTPREQ_A {
    #[doc = "0: A stop condition is not generated."]
    _0 = 0,
    #[doc = "1: A stop condition is generated"]
    _1 = 1,
}
impl From<IICSTPREQ_A> for bool {
    #[inline(always)]
    fn from(variant: IICSTPREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IICSTPREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICSTPREQ_A {
        match self.bits {
            false => IICSTPREQ_A::_0,
            true => IICSTPREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICSTPREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICSTPREQ_A::_1
    }
}
#[doc = "Field `IICSTPREQ` writer - Stop Condition Generation"]
pub type IICSTPREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, IICSTPREQ_A, O>;
impl<'a, const O: u8> IICSTPREQ_W<'a, O> {
    #[doc = "A stop condition is not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICSTPREQ_A::_0)
    }
    #[doc = "A stop condition is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICSTPREQ_A::_1)
    }
}
#[doc = "Field `IICSDAS` reader - SDA Output Select"]
pub type IICSDAS_R = crate::FieldReader<u8, IICSDAS_A>;
#[doc = "SDA Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IICSDAS_A {
    #[doc = "0: Serial data output"]
    _00 = 0,
    #[doc = "1: Generate a start, restart, or stop condition."]
    _01 = 1,
    #[doc = "2: Output the low level on the SDAn pin."]
    _10 = 2,
    #[doc = "3: Place the SDAn pin in the high-impedance state."]
    _11 = 3,
}
impl From<IICSDAS_A> for u8 {
    #[inline(always)]
    fn from(variant: IICSDAS_A) -> Self {
        variant as _
    }
}
impl IICSDAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICSDAS_A {
        match self.bits {
            0 => IICSDAS_A::_00,
            1 => IICSDAS_A::_01,
            2 => IICSDAS_A::_10,
            3 => IICSDAS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IICSDAS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IICSDAS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IICSDAS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IICSDAS_A::_11
    }
}
#[doc = "Field `IICSDAS` writer - SDA Output Select"]
pub type IICSDAS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ICR_SPEC, u8, IICSDAS_A, 2, O>;
impl<'a, const O: u8> IICSDAS_W<'a, O> {
    #[doc = "Serial data output"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(IICSDAS_A::_00)
    }
    #[doc = "Generate a start, restart, or stop condition."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(IICSDAS_A::_01)
    }
    #[doc = "Output the low level on the SDAn pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(IICSDAS_A::_10)
    }
    #[doc = "Place the SDAn pin in the high-impedance state."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(IICSDAS_A::_11)
    }
}
#[doc = "Field `IICSCLS` reader - SCL Output Select"]
pub type IICSCLS_R = crate::FieldReader<u8, IICSCLS_A>;
#[doc = "SCL Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IICSCLS_A {
    #[doc = "0: Serial clock output"]
    _00 = 0,
    #[doc = "1: Generate a start, restart, or stop condition."]
    _01 = 1,
    #[doc = "2: Output the low level on the SCLn pin."]
    _10 = 2,
    #[doc = "3: Place the SCLn pin in the high-impedance state."]
    _11 = 3,
}
impl From<IICSCLS_A> for u8 {
    #[inline(always)]
    fn from(variant: IICSCLS_A) -> Self {
        variant as _
    }
}
impl IICSCLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICSCLS_A {
        match self.bits {
            0 => IICSCLS_A::_00,
            1 => IICSCLS_A::_01,
            2 => IICSCLS_A::_10,
            3 => IICSCLS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IICSCLS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IICSCLS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IICSCLS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IICSCLS_A::_11
    }
}
#[doc = "Field `IICSCLS` writer - SCL Output Select"]
pub type IICSCLS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ICR_SPEC, u8, IICSCLS_A, 2, O>;
impl<'a, const O: u8> IICSCLS_W<'a, O> {
    #[doc = "Serial clock output"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(IICSCLS_A::_00)
    }
    #[doc = "Generate a start, restart, or stop condition."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(IICSCLS_A::_01)
    }
    #[doc = "Output the low level on the SCLn pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(IICSCLS_A::_10)
    }
    #[doc = "Place the SCLn pin in the high-impedance state."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(IICSCLS_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:4 - SDA Delay Output Select"]
    #[inline(always)]
    pub fn iicdl(&self) -> IICDL_R {
        IICDL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - IIC Interrupt Mode Select"]
    #[inline(always)]
    pub fn iicintm(&self) -> IICINTM_R {
        IICINTM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock Synchronization"]
    #[inline(always)]
    pub fn iiccsc(&self) -> IICCSC_R {
        IICCSC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - ACK Transmission Data"]
    #[inline(always)]
    pub fn iicackt(&self) -> IICACKT_R {
        IICACKT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Start Condition Generation"]
    #[inline(always)]
    pub fn iicstareq(&self) -> IICSTAREQ_R {
        IICSTAREQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Restart Condition Generation"]
    #[inline(always)]
    pub fn iicrstareq(&self) -> IICRSTAREQ_R {
        IICRSTAREQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Stop Condition Generation"]
    #[inline(always)]
    pub fn iicstpreq(&self) -> IICSTPREQ_R {
        IICSTPREQ_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - SDA Output Select"]
    #[inline(always)]
    pub fn iicsdas(&self) -> IICSDAS_R {
        IICSDAS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - SCL Output Select"]
    #[inline(always)]
    pub fn iicscls(&self) -> IICSCLS_R {
        IICSCLS_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SDA Delay Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn iicdl(&mut self) -> IICDL_W<0> {
        IICDL_W::new(self)
    }
    #[doc = "Bit 8 - IIC Interrupt Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn iicintm(&mut self) -> IICINTM_W<8> {
        IICINTM_W::new(self)
    }
    #[doc = "Bit 9 - Clock Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn iiccsc(&mut self) -> IICCSC_W<9> {
        IICCSC_W::new(self)
    }
    #[doc = "Bit 13 - ACK Transmission Data"]
    #[inline(always)]
    #[must_use]
    pub fn iicackt(&mut self) -> IICACKT_W<13> {
        IICACKT_W::new(self)
    }
    #[doc = "Bit 16 - Start Condition Generation"]
    #[inline(always)]
    #[must_use]
    pub fn iicstareq(&mut self) -> IICSTAREQ_W<16> {
        IICSTAREQ_W::new(self)
    }
    #[doc = "Bit 17 - Restart Condition Generation"]
    #[inline(always)]
    #[must_use]
    pub fn iicrstareq(&mut self) -> IICRSTAREQ_W<17> {
        IICRSTAREQ_W::new(self)
    }
    #[doc = "Bit 18 - Stop Condition Generation"]
    #[inline(always)]
    #[must_use]
    pub fn iicstpreq(&mut self) -> IICSTPREQ_W<18> {
        IICSTPREQ_W::new(self)
    }
    #[doc = "Bits 20:21 - SDA Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn iicsdas(&mut self) -> IICSDAS_W<20> {
        IICSDAS_W::new(self)
    }
    #[doc = "Bits 22:23 - SCL Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn iicscls(&mut self) -> IICSCLS_W<22> {
        IICSCLS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Simple IIC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
