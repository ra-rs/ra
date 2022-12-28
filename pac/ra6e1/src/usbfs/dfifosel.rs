#[doc = "Register `D%sFIFOSEL` reader"]
pub struct R(crate::R<DFIFOSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFIFOSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFIFOSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFIFOSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D%sFIFOSEL` writer"]
pub struct W(crate::W<DFIFOSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFIFOSEL_SPEC>;
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
impl From<crate::W<DFIFOSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFIFOSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURPIPE` reader - FIFO Port Access Pipe Specification"]
pub type CURPIPE_R = crate::FieldReader<u8, CURPIPE_A>;
#[doc = "FIFO Port Access Pipe Specification\n\nValue on reset: 0"]
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
#[doc = "Field `CURPIPE` writer - FIFO Port Access Pipe Specification"]
pub type CURPIPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, DFIFOSEL_SPEC, u8, CURPIPE_A, 4, O>;
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
pub type BIGEND_W<'a, const O: u8> = crate::BitWriter<'a, u16, DFIFOSEL_SPEC, BIGEND_A, O>;
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
#[doc = "Field `MBW` reader - FIFO Port Access Bit Width"]
pub type MBW_R = crate::BitReader<MBW_A>;
#[doc = "FIFO Port Access Bit Width\n\nValue on reset: 0"]
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
#[doc = "Field `MBW` writer - FIFO Port Access Bit Width"]
pub type MBW_W<'a, const O: u8> = crate::BitWriter<'a, u16, DFIFOSEL_SPEC, MBW_A, O>;
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
#[doc = "Field `DREQE` reader - DMA/DTC Transfer Request Enable"]
pub type DREQE_R = crate::BitReader<DREQE_A>;
#[doc = "DMA/DTC Transfer Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DREQE_A {
    #[doc = "0: Disable DMA/DTC transfer request"]
    _0 = 0,
    #[doc = "1: Enable DMA/DTC transfer request"]
    _1 = 1,
}
impl From<DREQE_A> for bool {
    #[inline(always)]
    fn from(variant: DREQE_A) -> Self {
        variant as u8 != 0
    }
}
impl DREQE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DREQE_A {
        match self.bits {
            false => DREQE_A::_0,
            true => DREQE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DREQE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DREQE_A::_1
    }
}
#[doc = "Field `DREQE` writer - DMA/DTC Transfer Request Enable"]
pub type DREQE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DFIFOSEL_SPEC, DREQE_A, O>;
impl<'a, const O: u8> DREQE_W<'a, O> {
    #[doc = "Disable DMA/DTC transfer request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DREQE_A::_0)
    }
    #[doc = "Enable DMA/DTC transfer request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DREQE_A::_1)
    }
}
#[doc = "Field `DCLRM` reader - Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read"]
pub type DCLRM_R = crate::BitReader<DCLRM_A>;
#[doc = "Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCLRM_A {
    #[doc = "0: Disable auto buffer clear mode"]
    _0 = 0,
    #[doc = "1: Enable auto buffer clear mode"]
    _1 = 1,
}
impl From<DCLRM_A> for bool {
    #[inline(always)]
    fn from(variant: DCLRM_A) -> Self {
        variant as u8 != 0
    }
}
impl DCLRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCLRM_A {
        match self.bits {
            false => DCLRM_A::_0,
            true => DCLRM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCLRM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCLRM_A::_1
    }
}
#[doc = "Field `DCLRM` writer - Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read"]
pub type DCLRM_W<'a, const O: u8> = crate::BitWriter<'a, u16, DFIFOSEL_SPEC, DCLRM_A, O>;
impl<'a, const O: u8> DCLRM_W<'a, O> {
    #[doc = "Disable auto buffer clear mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCLRM_A::_0)
    }
    #[doc = "Enable auto buffer clear mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCLRM_A::_1)
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
pub type REW_W<'a, const O: u8> = crate::BitWriter<'a, u16, DFIFOSEL_SPEC, REW_AW, O>;
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
    #[doc = "0: Clear DTLN\\[8:0\\]
bits in (CFIFOCTR.DTLN\\[8:0\\], D0FIFOCTR.DTLN\\[8:0\\], D1FIFOCTR.DTLN\\[8:0\\]) when all receive data is read from DnFIFO (after read of a single plane in double buffer mode)"]
    _0 = 0,
    #[doc = "1: Decrement DTLN\\[8:0\\]
bits each time receive data is read from DnFIFO"]
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
pub type RCNT_W<'a, const O: u8> = crate::BitWriter<'a, u16, DFIFOSEL_SPEC, RCNT_A, O>;
impl<'a, const O: u8> RCNT_W<'a, O> {
    #[doc = "Clear DTLN\\[8:0\\]
bits in (CFIFOCTR.DTLN\\[8:0\\], D0FIFOCTR.DTLN\\[8:0\\], D1FIFOCTR.DTLN\\[8:0\\]) when all receive data is read from DnFIFO (after read of a single plane in double buffer mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCNT_A::_0)
    }
    #[doc = "Decrement DTLN\\[8:0\\]
bits each time receive data is read from DnFIFO"]
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
    #[doc = "Bit 8 - FIFO Port Endian Control"]
    #[inline(always)]
    pub fn bigend(&self) -> BIGEND_R {
        BIGEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - FIFO Port Access Bit Width"]
    #[inline(always)]
    pub fn mbw(&self) -> MBW_R {
        MBW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA/DTC Transfer Request Enable"]
    #[inline(always)]
    pub fn dreqe(&self) -> DREQE_R {
        DREQE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read"]
    #[inline(always)]
    pub fn dclrm(&self) -> DCLRM_R {
        DCLRM_R::new(((self.bits >> 13) & 1) != 0)
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
    #[doc = "Bit 8 - FIFO Port Endian Control"]
    #[inline(always)]
    #[must_use]
    pub fn bigend(&mut self) -> BIGEND_W<8> {
        BIGEND_W::new(self)
    }
    #[doc = "Bit 10 - FIFO Port Access Bit Width"]
    #[inline(always)]
    #[must_use]
    pub fn mbw(&mut self) -> MBW_W<10> {
        MBW_W::new(self)
    }
    #[doc = "Bit 12 - DMA/DTC Transfer Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dreqe(&mut self) -> DREQE_W<12> {
        DREQE_W::new(self)
    }
    #[doc = "Bit 13 - Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read"]
    #[inline(always)]
    #[must_use]
    pub fn dclrm(&mut self) -> DCLRM_W<13> {
        DCLRM_W::new(self)
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
#[doc = "D%sFIFO Port Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfifosel](index.html) module"]
pub struct DFIFOSEL_SPEC;
impl crate::RegisterSpec for DFIFOSEL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dfifosel::R](R) reader structure"]
impl crate::Readable for DFIFOSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfifosel::W](W) writer structure"]
impl crate::Writable for DFIFOSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D%sFIFOSEL to value 0"]
impl crate::Resettable for DFIFOSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
