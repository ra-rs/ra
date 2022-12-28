#[doc = "Register `PRCR` reader"]
pub struct R(crate::R<PRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRCR` writer"]
pub struct W(crate::W<PRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRCR_SPEC>;
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
impl From<crate::W<PRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRC0` reader - Enables writing to the registers related to the clock generation circuit."]
pub type PRC0_R = crate::BitReader<PRC0_A>;
#[doc = "Enables writing to the registers related to the clock generation circuit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRC0_A {
    #[doc = "0: Writes protected."]
    _0 = 0,
    #[doc = "1: Writes not protected."]
    _1 = 1,
}
impl From<PRC0_A> for bool {
    #[inline(always)]
    fn from(variant: PRC0_A) -> Self {
        variant as u8 != 0
    }
}
impl PRC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRC0_A {
        match self.bits {
            false => PRC0_A::_0,
            true => PRC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRC0_A::_1
    }
}
#[doc = "Field `PRC0` writer - Enables writing to the registers related to the clock generation circuit."]
pub type PRC0_W<'a, const O: u8> = crate::BitWriter<'a, u16, PRCR_SPEC, PRC0_A, O>;
impl<'a, const O: u8> PRC0_W<'a, O> {
    #[doc = "Writes protected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRC0_A::_0)
    }
    #[doc = "Writes not protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRC0_A::_1)
    }
}
#[doc = "Field `PRC1` reader - Enables writing to the registers related to the operating modes, the low power consumption modes and the battery backup function."]
pub type PRC1_R = crate::BitReader<PRC1_A>;
#[doc = "Enables writing to the registers related to the operating modes, the low power consumption modes and the battery backup function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRC1_A {
    #[doc = "0: Writes protected."]
    _0 = 0,
    #[doc = "1: Writes not protected."]
    _1 = 1,
}
impl From<PRC1_A> for bool {
    #[inline(always)]
    fn from(variant: PRC1_A) -> Self {
        variant as u8 != 0
    }
}
impl PRC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRC1_A {
        match self.bits {
            false => PRC1_A::_0,
            true => PRC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRC1_A::_1
    }
}
#[doc = "Field `PRC1` writer - Enables writing to the registers related to the operating modes, the low power consumption modes and the battery backup function."]
pub type PRC1_W<'a, const O: u8> = crate::BitWriter<'a, u16, PRCR_SPEC, PRC1_A, O>;
impl<'a, const O: u8> PRC1_W<'a, O> {
    #[doc = "Writes protected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRC1_A::_0)
    }
    #[doc = "Writes not protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRC1_A::_1)
    }
}
#[doc = "Field `PRC3` reader - Enables writing to the registers related to the LVD."]
pub type PRC3_R = crate::BitReader<PRC3_A>;
#[doc = "Enables writing to the registers related to the LVD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRC3_A {
    #[doc = "0: Writes protected."]
    _0 = 0,
    #[doc = "1: Writes not protected."]
    _1 = 1,
}
impl From<PRC3_A> for bool {
    #[inline(always)]
    fn from(variant: PRC3_A) -> Self {
        variant as u8 != 0
    }
}
impl PRC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRC3_A {
        match self.bits {
            false => PRC3_A::_0,
            true => PRC3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRC3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRC3_A::_1
    }
}
#[doc = "Field `PRC3` writer - Enables writing to the registers related to the LVD."]
pub type PRC3_W<'a, const O: u8> = crate::BitWriter<'a, u16, PRCR_SPEC, PRC3_A, O>;
impl<'a, const O: u8> PRC3_W<'a, O> {
    #[doc = "Writes protected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRC3_A::_0)
    }
    #[doc = "Writes not protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRC3_A::_1)
    }
}
#[doc = "PRKEY Key Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRKEY_AW {
    #[doc = "90: Enables writing to the PRCR register."]
    _0X5A = 90,
}
impl From<PRKEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: PRKEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `PRKEY` writer - PRKEY Key Code"]
pub type PRKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PRCR_SPEC, u8, PRKEY_AW, 8, O>;
impl<'a, const O: u8> PRKEY_W<'a, O> {
    #[doc = "Enables writing to the PRCR register."]
    #[inline(always)]
    pub fn _0x5a(self) -> &'a mut W {
        self.variant(PRKEY_AW::_0X5A)
    }
}
impl R {
    #[doc = "Bit 0 - Enables writing to the registers related to the clock generation circuit."]
    #[inline(always)]
    pub fn prc0(&self) -> PRC0_R {
        PRC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables writing to the registers related to the operating modes, the low power consumption modes and the battery backup function."]
    #[inline(always)]
    pub fn prc1(&self) -> PRC1_R {
        PRC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables writing to the registers related to the LVD."]
    #[inline(always)]
    pub fn prc3(&self) -> PRC3_R {
        PRC3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables writing to the registers related to the clock generation circuit."]
    #[inline(always)]
    #[must_use]
    pub fn prc0(&mut self) -> PRC0_W<0> {
        PRC0_W::new(self)
    }
    #[doc = "Bit 1 - Enables writing to the registers related to the operating modes, the low power consumption modes and the battery backup function."]
    #[inline(always)]
    #[must_use]
    pub fn prc1(&mut self) -> PRC1_W<1> {
        PRC1_W::new(self)
    }
    #[doc = "Bit 3 - Enables writing to the registers related to the LVD."]
    #[inline(always)]
    #[must_use]
    pub fn prc3(&mut self) -> PRC3_W<3> {
        PRC3_W::new(self)
    }
    #[doc = "Bits 8:15 - PRKEY Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn prkey(&mut self) -> PRKEY_W<8> {
        PRKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prcr](index.html) module"]
pub struct PRCR_SPEC;
impl crate::RegisterSpec for PRCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [prcr::R](R) reader structure"]
impl crate::Readable for PRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prcr::W](W) writer structure"]
impl crate::Writable for PRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRCR to value 0"]
impl crate::Resettable for PRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
