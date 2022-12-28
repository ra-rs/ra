#[doc = "Register `HOCOWTCR` reader"]
pub struct R(crate::R<HOCOWTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOCOWTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOCOWTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOCOWTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOCOWTCR` writer"]
pub struct W(crate::W<HOCOWTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOCOWTCR_SPEC>;
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
impl From<crate::W<HOCOWTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOCOWTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTS` reader - HOCO wait time settingWaiting time (sec) = setting of the HSTS\\[2:0\\]
bits/fLOCO(Trimmed) + 3/fLOC(Untrimmed)"]
pub type HSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSTS` writer - HOCO wait time settingWaiting time (sec) = setting of the HSTS\\[2:0\\]
bits/fLOCO(Trimmed) + 3/fLOC(Untrimmed)"]
pub type HSTS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, HOCOWTCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - HOCO wait time settingWaiting time (sec) = setting of the HSTS\\[2:0\\]
bits/fLOCO(Trimmed) + 3/fLOC(Untrimmed)"]
    #[inline(always)]
    pub fn hsts(&self) -> HSTS_R {
        HSTS_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - HOCO wait time settingWaiting time (sec) = setting of the HSTS\\[2:0\\]
bits/fLOCO(Trimmed) + 3/fLOC(Untrimmed)"]
    #[inline(always)]
    #[must_use]
    pub fn hsts(&mut self) -> HSTS_W<0> {
        HSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High-speed on-chip oscillator wait control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hocowtcr](index.html) module"]
pub struct HOCOWTCR_SPEC;
impl crate::RegisterSpec for HOCOWTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hocowtcr::R](R) reader structure"]
impl crate::Readable for HOCOWTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hocowtcr::W](W) writer structure"]
impl crate::Writable for HOCOWTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOCOWTCR to value 0x02"]
impl crate::Resettable for HOCOWTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
