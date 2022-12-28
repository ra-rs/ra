#[doc = "Register `VBTBKR[%s]` reader"]
pub struct R(crate::R<VBTBKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBTBKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBTBKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBTBKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VBTBKR[%s]` writer"]
pub struct W(crate::W<VBTBKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VBTBKR_SPEC>;
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
impl From<crate::W<VBTBKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VBTBKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBTBKR` reader - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT. The value of this register is retained even when VCC is not powered but VBATT is powered. VBTBKR is initialized by VBATT selected voltage power-on-reset."]
pub type VBTBKR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBTBKR` writer - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT. The value of this register is retained even when VCC is not powered but VBATT is powered. VBTBKR is initialized by VBATT selected voltage power-on-reset."]
pub type VBTBKR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, VBTBKR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT. The value of this register is retained even when VCC is not powered but VBATT is powered. VBTBKR is initialized by VBATT selected voltage power-on-reset."]
    #[inline(always)]
    pub fn vbtbkr(&self) -> VBTBKR_R {
        VBTBKR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT. The value of this register is retained even when VCC is not powered but VBATT is powered. VBTBKR is initialized by VBATT selected voltage power-on-reset."]
    #[inline(always)]
    #[must_use]
    pub fn vbtbkr(&mut self) -> VBTBKR_W<0> {
        VBTBKR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VBATT Backup Register \\[%s\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbtbkr](index.html) module"]
pub struct VBTBKR_SPEC;
impl crate::RegisterSpec for VBTBKR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vbtbkr::R](R) reader structure"]
impl crate::Readable for VBTBKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vbtbkr::W](W) writer structure"]
impl crate::Writable for VBTBKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTBKR[%s]
to value 0"]
impl crate::Resettable for VBTBKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
