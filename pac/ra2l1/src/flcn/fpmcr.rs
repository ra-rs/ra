#[doc = "Register `FPMCR` reader"]
pub struct R(crate::R<FPMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPMCR` writer"]
pub struct W(crate::W<FPMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPMCR_SPEC>;
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
impl From<crate::W<FPMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMS0` reader - Flash Operating Mode Select 0"]
pub type FMS0_R = crate::BitReader<FMS0_A>;
#[doc = "Flash Operating Mode Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMS0_A {
    #[doc = "0: FMS1 = 0: Read mode FMS1 = 1: Data flash P/E mode."]
    _0 = 0,
    #[doc = "1: FMS1 = 0: Code flash P/E mode FMS1 = 1: Setting prohibited."]
    _1 = 1,
}
impl From<FMS0_A> for bool {
    #[inline(always)]
    fn from(variant: FMS0_A) -> Self {
        variant as u8 != 0
    }
}
impl FMS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMS0_A {
        match self.bits {
            false => FMS0_A::_0,
            true => FMS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FMS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FMS0_A::_1
    }
}
#[doc = "Field `FMS0` writer - Flash Operating Mode Select 0"]
pub type FMS0_W<'a, const O: u8> = crate::BitWriter<'a, u8, FPMCR_SPEC, FMS0_A, O>;
impl<'a, const O: u8> FMS0_W<'a, O> {
    #[doc = "FMS1 = 0: Read mode FMS1 = 1: Data flash P/E mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FMS0_A::_0)
    }
    #[doc = "FMS1 = 0: Code flash P/E mode FMS1 = 1: Setting prohibited."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FMS0_A::_1)
    }
}
#[doc = "Field `RPDIS` reader - Code Flash P/E Disable"]
pub type RPDIS_R = crate::BitReader<RPDIS_A>;
#[doc = "Code Flash P/E Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPDIS_A {
    #[doc = "0: Programming of the code flash is enabled"]
    _0 = 0,
    #[doc = "1: Programming of the code flash is disabled."]
    _1 = 1,
}
impl From<RPDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RPDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl RPDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPDIS_A {
        match self.bits {
            false => RPDIS_A::_0,
            true => RPDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPDIS_A::_1
    }
}
#[doc = "Field `RPDIS` writer - Code Flash P/E Disable"]
pub type RPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u8, FPMCR_SPEC, RPDIS_A, O>;
impl<'a, const O: u8> RPDIS_W<'a, O> {
    #[doc = "Programming of the code flash is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPDIS_A::_0)
    }
    #[doc = "Programming of the code flash is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPDIS_A::_1)
    }
}
#[doc = "Field `FMS1` reader - Flash Operating Mode Select 1"]
pub type FMS1_R = crate::BitReader<bool>;
#[doc = "Field `FMS1` writer - Flash Operating Mode Select 1"]
pub type FMS1_W<'a, const O: u8> = crate::BitWriter<'a, u8, FPMCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Flash Operating Mode Select 0"]
    #[inline(always)]
    pub fn fms0(&self) -> FMS0_R {
        FMS0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Code Flash P/E Disable"]
    #[inline(always)]
    pub fn rpdis(&self) -> RPDIS_R {
        RPDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Operating Mode Select 1"]
    #[inline(always)]
    pub fn fms1(&self) -> FMS1_R {
        FMS1_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Flash Operating Mode Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn fms0(&mut self) -> FMS0_W<1> {
        FMS0_W::new(self)
    }
    #[doc = "Bit 3 - Code Flash P/E Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rpdis(&mut self) -> RPDIS_W<3> {
        RPDIS_W::new(self)
    }
    #[doc = "Bit 4 - Flash Operating Mode Select 1"]
    #[inline(always)]
    #[must_use]
    pub fn fms1(&mut self) -> FMS1_W<4> {
        FMS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash P/E Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpmcr](index.html) module"]
pub struct FPMCR_SPEC;
impl crate::RegisterSpec for FPMCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fpmcr::R](R) reader structure"]
impl crate::Readable for FPMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpmcr::W](W) writer structure"]
impl crate::Writable for FPMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPMCR to value 0x08"]
impl crate::Resettable for FPMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
