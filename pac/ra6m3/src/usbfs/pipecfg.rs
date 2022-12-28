#[doc = "Register `PIPECFG` reader"]
pub struct R(crate::R<PIPECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIPECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIPECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIPECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIPECFG` writer"]
pub struct W(crate::W<PIPECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIPECFG_SPEC>;
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
impl From<crate::W<PIPECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIPECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPNUM` reader - Endpoint NumberThese bits specify the endpoint number for the selected pipe.Setting 0000b means unused pipe."]
pub type EPNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPNUM` writer - Endpoint NumberThese bits specify the endpoint number for the selected pipe.Setting 0000b means unused pipe."]
pub type EPNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PIPECFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DIR` reader - Transfer Direction"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Transfer Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Receiving direction"]
    _0 = 0,
    #[doc = "1: Transmitting direction"]
    _1 = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::_0,
            true => DIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIR_A::_1
    }
}
#[doc = "Field `DIR` writer - Transfer Direction"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u16, PIPECFG_SPEC, DIR_A, O>;
impl<'a, const O: u8> DIR_W<'a, O> {
    #[doc = "Receiving direction"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIR_A::_0)
    }
    #[doc = "Transmitting direction"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIR_A::_1)
    }
}
#[doc = "Field `SHTNAK` reader - Pipe Disabled at End of Transfer"]
pub type SHTNAK_R = crate::BitReader<SHTNAK_A>;
#[doc = "Pipe Disabled at End of Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHTNAK_A {
    #[doc = "0: Pipe assignment continued at the end of transfer"]
    _0 = 0,
    #[doc = "1: Pipe assignment disabled at the end of transfer"]
    _1 = 1,
}
impl From<SHTNAK_A> for bool {
    #[inline(always)]
    fn from(variant: SHTNAK_A) -> Self {
        variant as u8 != 0
    }
}
impl SHTNAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHTNAK_A {
        match self.bits {
            false => SHTNAK_A::_0,
            true => SHTNAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHTNAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHTNAK_A::_1
    }
}
#[doc = "Field `SHTNAK` writer - Pipe Disabled at End of Transfer"]
pub type SHTNAK_W<'a, const O: u8> = crate::BitWriter<'a, u16, PIPECFG_SPEC, SHTNAK_A, O>;
impl<'a, const O: u8> SHTNAK_W<'a, O> {
    #[doc = "Pipe assignment continued at the end of transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHTNAK_A::_0)
    }
    #[doc = "Pipe assignment disabled at the end of transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHTNAK_A::_1)
    }
}
#[doc = "Field `DBLB` reader - Double Buffer Mode"]
pub type DBLB_R = crate::BitReader<DBLB_A>;
#[doc = "Double Buffer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBLB_A {
    #[doc = "0: Single buffer"]
    _0 = 0,
    #[doc = "1: Double buffer"]
    _1 = 1,
}
impl From<DBLB_A> for bool {
    #[inline(always)]
    fn from(variant: DBLB_A) -> Self {
        variant as u8 != 0
    }
}
impl DBLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBLB_A {
        match self.bits {
            false => DBLB_A::_0,
            true => DBLB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBLB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBLB_A::_1
    }
}
#[doc = "Field `DBLB` writer - Double Buffer Mode"]
pub type DBLB_W<'a, const O: u8> = crate::BitWriter<'a, u16, PIPECFG_SPEC, DBLB_A, O>;
impl<'a, const O: u8> DBLB_W<'a, O> {
    #[doc = "Single buffer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBLB_A::_0)
    }
    #[doc = "Double buffer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBLB_A::_1)
    }
}
#[doc = "Field `BFRE` reader - BRDY Interrupt Operation Specification"]
pub type BFRE_R = crate::BitReader<BFRE_A>;
#[doc = "BRDY Interrupt Operation Specification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFRE_A {
    #[doc = "0: BRDY interrupt upon transmitting or receiving data"]
    _0 = 0,
    #[doc = "1: BRDY interrupt upon completion of reading data"]
    _1 = 1,
}
impl From<BFRE_A> for bool {
    #[inline(always)]
    fn from(variant: BFRE_A) -> Self {
        variant as u8 != 0
    }
}
impl BFRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFRE_A {
        match self.bits {
            false => BFRE_A::_0,
            true => BFRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFRE_A::_1
    }
}
#[doc = "Field `BFRE` writer - BRDY Interrupt Operation Specification"]
pub type BFRE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PIPECFG_SPEC, BFRE_A, O>;
impl<'a, const O: u8> BFRE_W<'a, O> {
    #[doc = "BRDY interrupt upon transmitting or receiving data"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFRE_A::_0)
    }
    #[doc = "BRDY interrupt upon completion of reading data"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFRE_A::_1)
    }
}
#[doc = "Field `TYPE` reader - Transfer Type"]
pub type TYPE_R = crate::FieldReader<u8, TYPE_A>;
#[doc = "Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TYPE_A {
    #[doc = "0: Pipe not used"]
    _00 = 0,
    #[doc = "1: Bulk transfer(PIPE1 and PIPE5) /Setting prohibited(PIPE6 to PIPE9)"]
    _01 = 1,
    #[doc = "2: Setting prohibited(PIPE1 and PIPE5) /Interrupt transfer(PIPE6 to PIPE9)"]
    _10 = 2,
    #[doc = "3: Isochronous transfer(PIPE1 and PIPE2) /Setting prohibited(PIPE3 to PIPE9)"]
    _11 = 3,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
    }
}
impl TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE_A {
        match self.bits {
            0 => TYPE_A::_00,
            1 => TYPE_A::_01,
            2 => TYPE_A::_10,
            3 => TYPE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TYPE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TYPE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TYPE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TYPE_A::_11
    }
}
#[doc = "Field `TYPE` writer - Transfer Type"]
pub type TYPE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, PIPECFG_SPEC, u8, TYPE_A, 2, O>;
impl<'a, const O: u8> TYPE_W<'a, O> {
    #[doc = "Pipe not used"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TYPE_A::_00)
    }
    #[doc = "Bulk transfer(PIPE1 and PIPE5) /Setting prohibited(PIPE6 to PIPE9)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TYPE_A::_01)
    }
    #[doc = "Setting prohibited(PIPE1 and PIPE5) /Interrupt transfer(PIPE6 to PIPE9)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TYPE_A::_10)
    }
    #[doc = "Isochronous transfer(PIPE1 and PIPE2) /Setting prohibited(PIPE3 to PIPE9)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TYPE_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:3 - Endpoint NumberThese bits specify the endpoint number for the selected pipe.Setting 0000b means unused pipe."]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Transfer Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Pipe Disabled at End of Transfer"]
    #[inline(always)]
    pub fn shtnak(&self) -> SHTNAK_R {
        SHTNAK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Double Buffer Mode"]
    #[inline(always)]
    pub fn dblb(&self) -> DBLB_R {
        DBLB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRDY Interrupt Operation Specification"]
    #[inline(always)]
    pub fn bfre(&self) -> BFRE_R {
        BFRE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Transfer Type"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint NumberThese bits specify the endpoint number for the selected pipe.Setting 0000b means unused pipe."]
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EPNUM_W<0> {
        EPNUM_W::new(self)
    }
    #[doc = "Bit 4 - Transfer Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    #[doc = "Bit 7 - Pipe Disabled at End of Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn shtnak(&mut self) -> SHTNAK_W<7> {
        SHTNAK_W::new(self)
    }
    #[doc = "Bit 9 - Double Buffer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dblb(&mut self) -> DBLB_W<9> {
        DBLB_W::new(self)
    }
    #[doc = "Bit 10 - BRDY Interrupt Operation Specification"]
    #[inline(always)]
    #[must_use]
    pub fn bfre(&mut self) -> BFRE_W<10> {
        BFRE_W::new(self)
    }
    #[doc = "Bits 14:15 - Transfer Type"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<14> {
        TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pipecfg](index.html) module"]
pub struct PIPECFG_SPEC;
impl crate::RegisterSpec for PIPECFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pipecfg::R](R) reader structure"]
impl crate::Readable for PIPECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pipecfg::W](W) writer structure"]
impl crate::Writable for PIPECFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIPECFG to value 0"]
impl crate::Resettable for PIPECFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
