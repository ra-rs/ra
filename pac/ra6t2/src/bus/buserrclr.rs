#[doc = "Register `BUS%sERRCLR` reader"]
pub struct R(crate::R<BUSERRCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSERRCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSERRCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSERRCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUS%sERRCLR` writer"]
pub struct W(crate::W<BUSERRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSERRCLR_SPEC>;
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
impl From<crate::W<BUSERRCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSERRCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLERRCLR` reader - Slave bus Error Clear"]
pub type SLERRCLR_R = crate::BitReader<bool>;
#[doc = "Field `SLERRCLR` writer - Slave bus Error Clear"]
pub type SLERRCLR_W<'a, const O: u8> = crate::BitWriter<'a, u8, BUSERRCLR_SPEC, bool, O>;
#[doc = "Field `STERRCLR` reader - Slave TrustZone filter Error Clear"]
pub type STERRCLR_R = crate::BitReader<bool>;
#[doc = "Field `STERRCLR` writer - Slave TrustZone filter Error Clear"]
pub type STERRCLR_W<'a, const O: u8> = crate::BitWriter<'a, u8, BUSERRCLR_SPEC, bool, O>;
#[doc = "Field `MMERRCLR` reader - Master MPU Error Clear"]
pub type MMERRCLR_R = crate::BitReader<bool>;
#[doc = "Field `MMERRCLR` writer - Master MPU Error Clear"]
pub type MMERRCLR_W<'a, const O: u8> = crate::BitWriter<'a, u8, BUSERRCLR_SPEC, bool, O>;
#[doc = "Field `ILERRCLR` reader - Illegal Address Access Error Clear"]
pub type ILERRCLR_R = crate::BitReader<bool>;
#[doc = "Field `ILERRCLR` writer - Illegal Address Access Error Clear"]
pub type ILERRCLR_W<'a, const O: u8> = crate::BitWriter<'a, u8, BUSERRCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Slave bus Error Clear"]
    #[inline(always)]
    pub fn slerrclr(&self) -> SLERRCLR_R {
        SLERRCLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave TrustZone filter Error Clear"]
    #[inline(always)]
    pub fn sterrclr(&self) -> STERRCLR_R {
        STERRCLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Master MPU Error Clear"]
    #[inline(always)]
    pub fn mmerrclr(&self) -> MMERRCLR_R {
        MMERRCLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Illegal Address Access Error Clear"]
    #[inline(always)]
    pub fn ilerrclr(&self) -> ILERRCLR_R {
        ILERRCLR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave bus Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn slerrclr(&mut self) -> SLERRCLR_W<0> {
        SLERRCLR_W::new(self)
    }
    #[doc = "Bit 1 - Slave TrustZone filter Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sterrclr(&mut self) -> STERRCLR_W<1> {
        STERRCLR_W::new(self)
    }
    #[doc = "Bit 3 - Master MPU Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mmerrclr(&mut self) -> MMERRCLR_W<3> {
        MMERRCLR_W::new(self)
    }
    #[doc = "Bit 4 - Illegal Address Access Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ilerrclr(&mut self) -> ILERRCLR_W<4> {
        ILERRCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BUS Error Clear Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buserrclr](index.html) module"]
pub struct BUSERRCLR_SPEC;
impl crate::RegisterSpec for BUSERRCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [buserrclr::R](R) reader structure"]
impl crate::Readable for BUSERRCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buserrclr::W](W) writer structure"]
impl crate::Writable for BUSERRCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUS%sERRCLR to value 0"]
impl crate::Resettable for BUSERRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
