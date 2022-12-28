#[doc = "Register `PIR` reader"]
pub struct R(crate::R<PIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIR` writer"]
pub struct W(crate::W<PIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIR_SPEC>;
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
impl From<crate::W<PIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDC` reader - MII/RMII Management Data Clock"]
pub type MDC_R = crate::BitReader<bool>;
#[doc = "Field `MDC` writer - MII/RMII Management Data Clock"]
pub type MDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIR_SPEC, bool, O>;
#[doc = "Field `MMD` reader - MII/RMII Management Mode"]
pub type MMD_R = crate::BitReader<MMD_A>;
#[doc = "MII/RMII Management Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMD_A {
    #[doc = "0: Read"]
    _0 = 0,
    #[doc = "1: Write."]
    _1 = 1,
}
impl From<MMD_A> for bool {
    #[inline(always)]
    fn from(variant: MMD_A) -> Self {
        variant as u8 != 0
    }
}
impl MMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMD_A {
        match self.bits {
            false => MMD_A::_0,
            true => MMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MMD_A::_1
    }
}
#[doc = "Field `MMD` writer - MII/RMII Management Mode"]
pub type MMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIR_SPEC, MMD_A, O>;
impl<'a, const O: u8> MMD_W<'a, O> {
    #[doc = "Read"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MMD_A::_0)
    }
    #[doc = "Write."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MMD_A::_1)
    }
}
#[doc = "Field `MDO` reader - MII/RMII Management Data-Out"]
pub type MDO_R = crate::BitReader<bool>;
#[doc = "Field `MDO` writer - MII/RMII Management Data-Out"]
pub type MDO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIR_SPEC, bool, O>;
#[doc = "Field `MDI` reader - MII/RMII Management Data-In"]
pub type MDI_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - MII/RMII Management Data Clock"]
    #[inline(always)]
    pub fn mdc(&self) -> MDC_R {
        MDC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII/RMII Management Mode"]
    #[inline(always)]
    pub fn mmd(&self) -> MMD_R {
        MMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MII/RMII Management Data-Out"]
    #[inline(always)]
    pub fn mdo(&self) -> MDO_R {
        MDO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MII/RMII Management Data-In"]
    #[inline(always)]
    pub fn mdi(&self) -> MDI_R {
        MDI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MII/RMII Management Data Clock"]
    #[inline(always)]
    #[must_use]
    pub fn mdc(&mut self) -> MDC_W<0> {
        MDC_W::new(self)
    }
    #[doc = "Bit 1 - MII/RMII Management Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mmd(&mut self) -> MMD_W<1> {
        MMD_W::new(self)
    }
    #[doc = "Bit 2 - MII/RMII Management Data-Out"]
    #[inline(always)]
    #[must_use]
    pub fn mdo(&mut self) -> MDO_W<2> {
        MDO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Interface Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pir](index.html) module"]
pub struct PIR_SPEC;
impl crate::RegisterSpec for PIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pir::R](R) reader structure"]
impl crate::Readable for PIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pir::W](W) writer structure"]
impl crate::Writable for PIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIR to value 0"]
impl crate::Resettable for PIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
