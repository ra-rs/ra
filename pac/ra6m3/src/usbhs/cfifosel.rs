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
#[doc = "Field `CURPIPE` reader - FIFO Port Access Pipe Specification"]
pub type CURPIPE_R = crate::FieldReader<u8, CURPIPE_A>;
#[doc = "FIFO Port Access Pipe Specification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CURPIPE_A {
    #[doc = "0: DCP"]
    _0000 = 0,
    #[doc = "1: PIPE1"]
    _0001 = 1,
    #[doc = "2: PIPE2"]
    _0010 = 2,
    #[doc = "3: PIPE3"]
    _0011 = 3,
    #[doc = "4: PIPE4"]
    _0100 = 4,
    #[doc = "5: PIPE5"]
    _0101 = 5,
    #[doc = "6: PIPE6"]
    _0110 = 6,
    #[doc = "7: PIPE7"]
    _0111 = 7,
    #[doc = "8: PIPE8"]
    _1000 = 8,
    #[doc = "9: PIPE9"]
    _1001 = 9,
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
            0 => Some(CURPIPE_A::_0000),
            1 => Some(CURPIPE_A::_0001),
            2 => Some(CURPIPE_A::_0010),
            3 => Some(CURPIPE_A::_0011),
            4 => Some(CURPIPE_A::_0100),
            5 => Some(CURPIPE_A::_0101),
            6 => Some(CURPIPE_A::_0110),
            7 => Some(CURPIPE_A::_0111),
            8 => Some(CURPIPE_A::_1000),
            9 => Some(CURPIPE_A::_1001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == CURPIPE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CURPIPE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == CURPIPE_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == CURPIPE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CURPIPE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == CURPIPE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == CURPIPE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == CURPIPE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CURPIPE_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == CURPIPE_A::_1001
    }
}
#[doc = "Field `CURPIPE` writer - FIFO Port Access Pipe Specification"]
pub type CURPIPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CFIFOSEL_SPEC, u8, CURPIPE_A, 4, O>;
impl<'a, const O: u8> CURPIPE_W<'a, O> {
    #[doc = "DCP"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0000)
    }
    #[doc = "PIPE1"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0001)
    }
    #[doc = "PIPE2"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0010)
    }
    #[doc = "PIPE3"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0011)
    }
    #[doc = "PIPE4"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0100)
    }
    #[doc = "PIPE5"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0101)
    }
    #[doc = "PIPE6"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0110)
    }
    #[doc = "PIPE7"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0111)
    }
    #[doc = "PIPE8"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(CURPIPE_A::_1000)
    }
    #[doc = "PIPE9"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(CURPIPE_A::_1001)
    }
}
#[doc = "Field `ISEL` reader - FIFO Port Access Direction when DCP is Selected"]
pub type ISEL_R = crate::BitReader<ISEL_A>;
#[doc = "FIFO Port Access Direction when DCP is Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISEL_A {
    #[doc = "0: Select reading from the FIFO buffer"]
    _0 = 0,
    #[doc = "1: Select writing to the FIFO buffer."]
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
#[doc = "Field `ISEL` writer - FIFO Port Access Direction when DCP is Selected"]
pub type ISEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CFIFOSEL_SPEC, ISEL_A, O>;
impl<'a, const O: u8> ISEL_W<'a, O> {
    #[doc = "Select reading from the FIFO buffer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISEL_A::_0)
    }
    #[doc = "Select writing to the FIFO buffer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISEL_A::_1)
    }
}
#[doc = "Field `BIGEND` reader - FIFO Port Endian Control"]
pub type BIGEND_R = crate::BitReader<BIGEND_A>;
#[doc = "FIFO Port Endian Control\n\nValue on reset: 0"]
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
#[doc = "Field `BIGEND` writer - FIFO Port Endian Control"]
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
pub type MBW_R = crate::FieldReader<u8, MBW_A>;
#[doc = "CFIFO Port Access Bit Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MBW_A {
    #[doc = "0: 8-bit width"]
    _00 = 0,
    #[doc = "1: 16-bit width"]
    _01 = 1,
    #[doc = "2: 32-bit width"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<MBW_A> for u8 {
    #[inline(always)]
    fn from(variant: MBW_A) -> Self {
        variant as _
    }
}
impl MBW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBW_A {
        match self.bits {
            0 => MBW_A::_00,
            1 => MBW_A::_01,
            2 => MBW_A::_10,
            3 => MBW_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MBW_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MBW_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MBW_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MBW_A::_11
    }
}
#[doc = "Field `MBW` writer - CFIFO Port Access Bit Width"]
pub type MBW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CFIFOSEL_SPEC, u8, MBW_A, 2, O>;
impl<'a, const O: u8> MBW_W<'a, O> {
    #[doc = "8-bit width"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MBW_A::_00)
    }
    #[doc = "16-bit width"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MBW_A::_01)
    }
    #[doc = "32-bit width"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MBW_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MBW_A::_11)
    }
}
#[doc = "Buffer Pointer Rewind\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REW_AW {
    #[doc = "0: Do not rewind buffer pointer (Writing 0 has no effect.)"]
    _0 = 0,
    #[doc = "1: Rewind buffer pointer."]
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
    #[doc = "Do not rewind buffer pointer (Writing 0 has no effect.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REW_AW::_0)
    }
    #[doc = "Rewind buffer pointer."]
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
    #[doc = "0: Clear DTLN\\[11:0\\]
flags in the FIFO port control register to 000h when all receive data is read from CFIFO"]
    _0 = 0,
    #[doc = "1: Decrement DTLN\\[11:0\\]
flags each time receive data is read from CFIFO."]
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
    #[doc = "Clear DTLN\\[11:0\\]
flags in the FIFO port control register to 000h when all receive data is read from CFIFO"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCNT_A::_0)
    }
    #[doc = "Decrement DTLN\\[11:0\\]
flags each time receive data is read from CFIFO."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCNT_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - FIFO Port Access Pipe Specification"]
    #[inline(always)]
    pub fn curpipe(&self) -> CURPIPE_R {
        CURPIPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - FIFO Port Access Direction when DCP is Selected"]
    #[inline(always)]
    pub fn isel(&self) -> ISEL_R {
        ISEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO Port Endian Control"]
    #[inline(always)]
    pub fn bigend(&self) -> BIGEND_R {
        BIGEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - CFIFO Port Access Bit Width"]
    #[inline(always)]
    pub fn mbw(&self) -> MBW_R {
        MBW_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 15 - Read Count Mode"]
    #[inline(always)]
    pub fn rcnt(&self) -> RCNT_R {
        RCNT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FIFO Port Access Pipe Specification"]
    #[inline(always)]
    #[must_use]
    pub fn curpipe(&mut self) -> CURPIPE_W<0> {
        CURPIPE_W::new(self)
    }
    #[doc = "Bit 5 - FIFO Port Access Direction when DCP is Selected"]
    #[inline(always)]
    #[must_use]
    pub fn isel(&mut self) -> ISEL_W<5> {
        ISEL_W::new(self)
    }
    #[doc = "Bit 8 - FIFO Port Endian Control"]
    #[inline(always)]
    #[must_use]
    pub fn bigend(&mut self) -> BIGEND_W<8> {
        BIGEND_W::new(self)
    }
    #[doc = "Bits 10:11 - CFIFO Port Access Bit Width"]
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
