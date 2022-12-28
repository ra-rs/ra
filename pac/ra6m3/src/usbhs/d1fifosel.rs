#[doc = "Register `D1FIFOSEL` reader"]
pub struct R(crate::R<D1FIFOSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D1FIFOSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D1FIFOSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D1FIFOSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D1FIFOSEL` writer"]
pub struct W(crate::W<D1FIFOSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D1FIFOSEL_SPEC>;
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
impl From<crate::W<D1FIFOSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D1FIFOSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURPIPE` reader - FIFO Port Access Pipe Specification"]
pub type CURPIPE_R = crate::FieldReader<u8, CURPIPE_A>;
#[doc = "FIFO Port Access Pipe Specification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CURPIPE_A {
    #[doc = "0: No pipe specified"]
    _0000 = 0,
    #[doc = "1: Pipe 1"]
    _0001 = 1,
    #[doc = "2: Pipe 2"]
    _0010 = 2,
    #[doc = "3: Pipe 3"]
    _0011 = 3,
    #[doc = "4: Pipe 4"]
    _0100 = 4,
    #[doc = "5: Pipe 5"]
    _0101 = 5,
    #[doc = "6: Pipe 6"]
    _0110 = 6,
    #[doc = "7: Pipe 7"]
    _0111 = 7,
    #[doc = "8: Pipe 8"]
    _1000 = 8,
    #[doc = "9: Pipe 9"]
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
    crate::FieldWriter<'a, u16, D1FIFOSEL_SPEC, u8, CURPIPE_A, 4, O>;
impl<'a, const O: u8> CURPIPE_W<'a, O> {
    #[doc = "No pipe specified"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0000)
    }
    #[doc = "Pipe 1"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0001)
    }
    #[doc = "Pipe 2"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0010)
    }
    #[doc = "Pipe 3"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0011)
    }
    #[doc = "Pipe 4"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0100)
    }
    #[doc = "Pipe 5"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0101)
    }
    #[doc = "Pipe 6"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0110)
    }
    #[doc = "Pipe 7"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(CURPIPE_A::_0111)
    }
    #[doc = "Pipe 8"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(CURPIPE_A::_1000)
    }
    #[doc = "Pipe 9"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(CURPIPE_A::_1001)
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
pub type BIGEND_W<'a, const O: u8> = crate::BitWriter<'a, u16, D1FIFOSEL_SPEC, BIGEND_A, O>;
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
pub type MBW_R = crate::FieldReader<u8, MBW_A>;
#[doc = "FIFO Port Access Bit Width\n\nValue on reset: 0"]
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
#[doc = "Field `MBW` writer - FIFO Port Access Bit Width"]
pub type MBW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, D1FIFOSEL_SPEC, u8, MBW_A, 2, O>;
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
#[doc = "Field `DREQE` reader - UCL_Dx_DREQ Signal Output Enable"]
pub type DREQE_R = crate::BitReader<DREQE_A>;
#[doc = "UCL_Dx_DREQ Signal Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DREQE_A {
    #[doc = "0: Disables the output"]
    _0 = 0,
    #[doc = "1: Enables the output"]
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
#[doc = "Field `DREQE` writer - UCL_Dx_DREQ Signal Output Enable"]
pub type DREQE_W<'a, const O: u8> = crate::BitWriter<'a, u16, D1FIFOSEL_SPEC, DREQE_A, O>;
impl<'a, const O: u8> DREQE_W<'a, O> {
    #[doc = "Disables the output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DREQE_A::_0)
    }
    #[doc = "Enables the output"]
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
    #[doc = "0: Auto buffer clear mode is disabled"]
    _0 = 0,
    #[doc = "1: Auto buffer clear mode is enabled"]
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
pub type DCLRM_W<'a, const O: u8> = crate::BitWriter<'a, u16, D1FIFOSEL_SPEC, DCLRM_A, O>;
impl<'a, const O: u8> DCLRM_W<'a, O> {
    #[doc = "Auto buffer clear mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCLRM_A::_0)
    }
    #[doc = "Auto buffer clear mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCLRM_A::_1)
    }
}
#[doc = "Buffer Pointer Rewind\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REW_AW {
    #[doc = "0: The buffer pointer is not rewound"]
    _0 = 0,
    #[doc = "1: The buffer pointer is rewound"]
    _1 = 1,
}
impl From<REW_AW> for bool {
    #[inline(always)]
    fn from(variant: REW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REW` writer - Buffer Pointer Rewind"]
pub type REW_W<'a, const O: u8> = crate::BitWriter<'a, u16, D1FIFOSEL_SPEC, REW_AW, O>;
impl<'a, const O: u8> REW_W<'a, O> {
    #[doc = "The buffer pointer is not rewound"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REW_AW::_0)
    }
    #[doc = "The buffer pointer is rewound"]
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
    #[doc = "0: The DTLN bits are cleared when all of the receive data has been read from the CFIFO"]
    _0 = 0,
    #[doc = "1: The DTLN bits are decremented each time the receive data is read from the CFIFO"]
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
pub type RCNT_W<'a, const O: u8> = crate::BitWriter<'a, u16, D1FIFOSEL_SPEC, RCNT_A, O>;
impl<'a, const O: u8> RCNT_W<'a, O> {
    #[doc = "The DTLN bits are cleared when all of the receive data has been read from the CFIFO"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCNT_A::_0)
    }
    #[doc = "The DTLN bits are decremented each time the receive data is read from the CFIFO"]
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
    #[doc = "Bits 10:11 - FIFO Port Access Bit Width"]
    #[inline(always)]
    pub fn mbw(&self) -> MBW_R {
        MBW_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - UCL_Dx_DREQ Signal Output Enable"]
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
    #[doc = "Bits 10:11 - FIFO Port Access Bit Width"]
    #[inline(always)]
    #[must_use]
    pub fn mbw(&mut self) -> MBW_W<10> {
        MBW_W::new(self)
    }
    #[doc = "Bit 12 - UCL_Dx_DREQ Signal Output Enable"]
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
#[doc = "D1FIFO Port Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d1fifosel](index.html) module"]
pub struct D1FIFOSEL_SPEC;
impl crate::RegisterSpec for D1FIFOSEL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [d1fifosel::R](R) reader structure"]
impl crate::Readable for D1FIFOSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d1fifosel::W](W) writer structure"]
impl crate::Writable for D1FIFOSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D1FIFOSEL to value 0"]
impl crate::Resettable for D1FIFOSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
