#[doc = "Register `CMRLG` reader"]
pub struct R(crate::R<CMRLG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMRLG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMRLG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMRLG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMRLG` writer"]
pub struct W(crate::W<CMRLG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMRLG_SPEC>;
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
impl From<crate::W<CMRLG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMRLG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRLG` reader - Max Read Length"]
pub type MRLG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MRLG` writer - Max Read Length"]
pub type MRLG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMRLG_SPEC, u16, u16, 16, O>;
#[doc = "Field `IBIPSZ` reader - IBI Payload Size"]
pub type IBIPSZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIPSZ` writer - IBI Payload Size"]
pub type IBIPSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMRLG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:15 - Max Read Length"]
    #[inline(always)]
    pub fn mrlg(&self) -> MRLG_R {
        MRLG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - IBI Payload Size"]
    #[inline(always)]
    pub fn ibipsz(&self) -> IBIPSZ_R {
        IBIPSZ_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Max Read Length"]
    #[inline(always)]
    #[must_use]
    pub fn mrlg(&mut self) -> MRLG_W<0> {
        MRLG_W::new(self)
    }
    #[doc = "Bits 16:23 - IBI Payload Size"]
    #[inline(always)]
    #[must_use]
    pub fn ibipsz(&mut self) -> IBIPSZ_W<16> {
        IBIPSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCC Max Read Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmrlg](index.html) module"]
pub struct CMRLG_SPEC;
impl crate::RegisterSpec for CMRLG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmrlg::R](R) reader structure"]
impl crate::Readable for CMRLG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmrlg::W](W) writer structure"]
impl crate::Writable for CMRLG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMRLG to value 0"]
impl crate::Resettable for CMRLG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
