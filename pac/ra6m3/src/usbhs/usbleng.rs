#[doc = "Register `USBLENG` reader"]
pub struct R(crate::R<USBLENG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBLENG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBLENG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBLENG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBLENG` writer"]
pub struct W(crate::W<USBLENG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBLENG_SPEC>;
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
impl From<crate::W<USBLENG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBLENG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WLENGTH` reader - Value of USB request wLength Finction controller selected : read-only Host controller selected : read-write"]
pub type WLENGTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WLENGTH` writer - Value of USB request wLength Finction controller selected : read-only Host controller selected : read-write"]
pub type WLENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, u16, USBLENG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Value of USB request wLength Finction controller selected : read-only Host controller selected : read-write"]
    #[inline(always)]
    pub fn wlength(&self) -> WLENGTH_R {
        WLENGTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value of USB request wLength Finction controller selected : read-only Host controller selected : read-write"]
    #[inline(always)]
    #[must_use]
    pub fn wlength(&mut self) -> WLENGTH_W<0> {
        WLENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Request Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbleng](index.html) module"]
pub struct USBLENG_SPEC;
impl crate::RegisterSpec for USBLENG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbleng::R](R) reader structure"]
impl crate::Readable for USBLENG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbleng::W](W) writer structure"]
impl crate::Writable for USBLENG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBLENG to value 0"]
impl crate::Resettable for USBLENG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
