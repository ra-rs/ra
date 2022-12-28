#[doc = "Register `CFIFOSEL` reader"]
pub struct R(crate::R<CFIFOSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFIFOSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFIFOSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFIFOSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFIFOSEL` writer"]
pub struct W(crate::W<CFIFOSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFIFOSEL_SPEC>;
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
impl From<crate::W<CFIFOSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFIFOSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURPIPE` reader - CFIFO Port Access Pipe Specification"]
pub type CURPIPE_R = crate::FieldReader<u8, CURPIPE_A>;
#[doc = "CFIFO Port Access Pipe Specification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CURPIPE_A {
    #[doc = "0: Default Control Pipe"]
    _0X0 = 0,
    #[doc = "1: Pipe 1"]
    _0X1 = 1,
    #[doc = "2: Pipe 2"]
    _0X2 = 2,
    #[doc = "3: Pipe 3"]
    _0X3 = 3,
    #[doc = "4: Pipe 4"]
    _0X4 = 4,
    #[doc = "5: Pipe 5"]
    _0X5 = 5,
    #[doc = "6: Pipe 6"]
    _0X6 = 6,
    #[doc = "7: Pipe 7"]
    _0X7 = 7,
    #[doc = "8: Pipe 8"]
    _0X8 = 8,
    #[doc = "9: Pipe 9"]
    _0X9 = 9,
}
impl From<CURPIPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CURPIPE_A) -> Self {
        variant as _
    }
}
impl CURPIPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CURPIPE_A> {
        match self.bits {
            0 => Some(CURPIPE_A::_0X0),
            1 => Some(CURPIPE_A::_0X1),
            2 => Some(CURPIPE_A::_0X2),
            3 => Some(CURPIPE_A::_0X3),
            4 => Some(CURPIPE_A::_0X4),
            5 => Some(CURPIPE_A::_0X5),
            6 => Some(CURPIPE_A::_0X6),
            7 => Some(CURPIPE_A::_0X7),
            8 => Some(CURPIPE_A::_0X8),
            9 => Some(CURPIPE_A::_0X9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == CURPIPE_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == CURPIPE_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == CURPIPE_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == CURPIPE_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == CURPIPE_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == CURPIPE_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == CURPIPE_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == CURPIPE_A::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == CURPIPE_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == CURPIPE_A::_0X9
    }
}
#[doc = "Field `CURPIPE` writer - CFIFO Port Access Pipe Specification"]
pub type CURPIPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CFIFOSEL_SPEC, u8, CURPIPE_A, 4, O>;
impl<'a, const O: u8> CURPIPE_W<'a, O> {
    #[doc = "Default Control Pipe"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0X0)
    }
    #[doc = "Pipe 1"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0X1)
    }
    #[doc = "Pipe 2"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0X2)
    }
    #[doc = "Pipe 3"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0X3)
    }
    #[doc = "Pipe 4"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0X4)
    }
    #[doc = "Pipe 5"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0X5)
    }
    #[doc = "Pipe 6"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0X6)
    }
    #[doc = "Pipe 7"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0X7)
    }
    #[doc = "Pipe 8"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0X8)
    }
    #[doc = "Pipe 9"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0X9)
    }
}
#[doc = "Field `ISEL` reader - CFIFO Port Access Direction When DCP Is Selected"]
pub type ISEL_R = crate::BitReader<ISEL_A>;
#[doc = "CFIFO Port Access Direction When DCP Is Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISEL_A {
    #[doc = "0: Select reading from the FIFO buffer"]
    _0 = 0,
    #[doc = "1: Select writing to the FIFO buffer"]
    _1 = 1,
}
impl From<ISEL_A> for bool {
    #[inline(always)]
    fn from(variant: ISEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISEL_A {
        match self.bits {
            false => ISEL_A::_0,
            true => ISEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISEL_A::_1
    }
}
#[doc = "Field `ISEL` writer - CFIFO Port Access Direction When DCP Is Selected"]
pub type ISEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CFIFOSEL_SPEC, ISEL_A, O>;
impl<'a, const O: u8> ISEL_W<'a, O> {
    #[doc = "Select reading from the FIFO buffer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISEL_A::_0)
    }
    #[doc = "Select writing to the FIFO buffer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISEL_A::_1)
    }
}
#[doc = "Field `BIGEND` reader - CFIFO Port Endian Control"]
pub type BIGEND_R = crate::BitReader<BIGEND_A>;
#[doc = "CFIFO Port Endian Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIGEND_A {
    #[doc = "0: Little endian"]
    _0 = 0,
    #[doc = "1: Big endian"]
    _1 = 1,
}
impl From<BIGEND_A> for bool {
    #[inline(always)]
    fn from(variant: BIGEND_A) -> Self {
        variant as u8 != 0
    }
}
impl BIGEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIGEND_A {
        match self.bits {
            false => BIGEND_A::_0,
            true => BIGEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BIGEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BIGEND_A::_1
    }
}
#[doc = "Field `BIGEND` writer - CFIFO Port Endian Control"]
pub type BIGEND_W<'a, const O: u8> = crate::BitWriter<'a, u16, CFIFOSEL_SPEC, BIGEND_A, O>;
impl<'a, const O: u8> BIGEND_W<'a, O> {
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BIGEND_A::_0)
    }
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BIGEND_A::_1)
    }
}
#[doc = "Field `MBW` reader - CFIFO Port Access Bit Width"]
pub type MBW_R = crate::BitReader<MBW_A>;
#[doc = "CFIFO Port Access Bit Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBW_A {
    #[doc = "0: 8-bit width"]
    _0 = 0,
    #[doc = "1: 16-bit width"]
    _1 = 1,
}
impl From<MBW_A> for bool {
    #[inline(always)]
    fn from(variant: MBW_A) -> Self {
        variant as u8 != 0
    }
}
impl MBW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBW_A {
        match self.bits {
            false => MBW_A::_0,
            true => MBW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MBW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MBW_A::_1
    }
}
#[doc = "Field `MBW` writer - CFIFO Port Access Bit Width"]
pub type MBW_W<'a, const O: u8> = crate::BitWriter<'a, u16, CFIFOSEL_SPEC, MBW_A, O>;
impl<'a, const O: u8> MBW_W<'a, O> {
    #[doc = "8-bit width"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MBW_A::_0)
    }
    #[doc = "16-bit width"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MBW_A::_1)
    }
}
#[doc = "Buffer Pointer Rewind\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REW_AW {
    #[doc = "0: Do not rewind buffer pointer"]
    _0 = 0,
    #[doc = "1: Rewind buffer pointer"]
    _1 = 1,
}
impl From<REW_AW> for bool {
    #[inline(always)]
    fn from(variant: REW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REW` writer - Buffer Pointer Rewind"]
pub type REW_W<'a, const O: u8> = crate::BitWriter<'a, u16, CFIFOSEL_SPEC, REW_AW, O>;
impl<'a, const O: u8> REW_W<'a, O> {
    #[doc = "Do not rewind buffer pointer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REW_AW::_0)
    }
    #[doc = "Rewind buffer pointer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REW_AW::_1)
    }
}
#[doc = "Field `RCNT` reader - Read Count Mode"]
pub type RCNT_R = crate::BitReader<RCNT_A>;
#[doc = "Read Count Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCNT_A {
    #[doc = "0: The DTLN\\[8:0\\]
bits (CFIFOCTR.DTLN\\[8:0\\], D0FIFOCTR.DTLN\\[8:0\\], D1FIFOCTR.DTLN\\[8:0\\]) are cleared when all receive data is read from the CFIFO. In double buffer mode, the DTLN\\[8:0\\]
value is cleared when all data is read from only a single plane."]
    _0 = 0,
    #[doc = "1: The DTLN\\[8:0\\]
bits are decremented each time the receive data is read from the CFIFO."]
    _1 = 1,
}
impl From<RCNT_A> for bool {
    #[inline(always)]
    fn from(variant: RCNT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCNT_A {
        match self.bits {
            false => RCNT_A::_0,
            true => RCNT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCNT_A::_1
    }
}
#[doc = "Field `RCNT` writer - Read Count Mode"]
pub type RCNT_W<'a, const O: u8> = crate::BitWriter<'a, u16, CFIFOSEL_SPEC, RCNT_A, O>;
impl<'a, const O: u8> RCNT_W<'a, O> {
    #[doc = "The DTLN\\[8:0\\]
bits (CFIFOCTR.DTLN\\[8:0\\], D0FIFOCTR.DTLN\\[8:0\\], D1FIFOCTR.DTLN\\[8:0\\]) are cleared when all receive data is read from the CFIFO. In double buffer mode, the DTLN\\[8:0\\]
value is cleared when all data is read from only a single plane."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCNT_A::_0)
    }
    #[doc = "The DTLN\\[8:0\\]
bits are decremented each time the receive data is read from the CFIFO."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCNT_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - CFIFO Port Access Pipe Specification"]
    #[inline(always)]
    pub fn curpipe(&self) -> CURPIPE_R {
        CURPIPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - CFIFO Port Access Direction When DCP Is Selected"]
    #[inline(always)]
    pub fn isel(&self) -> ISEL_R {
        ISEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CFIFO Port Endian Control"]
    #[inline(always)]
    pub fn bigend(&self) -> BIGEND_R {
        BIGEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - CFIFO Port Access Bit Width"]
    #[inline(always)]
    pub fn mbw(&self) -> MBW_R {
        MBW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Read Count Mode"]
    #[inline(always)]
    pub fn rcnt(&self) -> RCNT_R {
        RCNT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - CFIFO Port Access Pipe Specification"]
    #[inline(always)]
    #[must_use]
    pub fn curpipe(&mut self) -> CURPIPE_W<0> {
        CURPIPE_W::new(self)
    }
    #[doc = "Bit 5 - CFIFO Port Access Direction When DCP Is Selected"]
    #[inline(always)]
    #[must_use]
    pub fn isel(&mut self) -> ISEL_W<5> {
        ISEL_W::new(self)
    }
    #[doc = "Bit 8 - CFIFO Port Endian Control"]
    #[inline(always)]
    #[must_use]
    pub fn bigend(&mut self) -> BIGEND_W<8> {
        BIGEND_W::new(self)
    }
    #[doc = "Bit 10 - CFIFO Port Access Bit Width"]
    #[inline(always)]
    #[must_use]
    pub fn mbw(&mut self) -> MBW_W<10> {
        MBW_W::new(self)
    }
    #[doc = "Bit 14 - Buffer Pointer Rewind"]
    #[inline(always)]
    #[must_use]
    pub fn rew(&mut self) -> REW_W<14> {
        REW_W::new(self)
    }
    #[doc = "Bit 15 - Read Count Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rcnt(&mut self) -> RCNT_W<15> {
        RCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFIFO Port Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfifosel](index.html) module"]
pub struct CFIFOSEL_SPEC;
impl crate::RegisterSpec for CFIFOSEL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cfifosel::R](R) reader structure"]
impl crate::Readable for CFIFOSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfifosel::W](W) writer structure"]
impl crate::Writable for CFIFOSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFIFOSEL to value 0"]
impl crate::Resettable for CFIFOSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
