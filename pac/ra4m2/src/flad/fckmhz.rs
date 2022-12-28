#[doc = "Register `FCKMHZ` reader"]
pub struct R(crate::R<FCKMHZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCKMHZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCKMHZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCKMHZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCKMHZ` writer"]
pub struct W(crate::W<FCKMHZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCKMHZ_SPEC>;
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
impl From<crate::W<FCKMHZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCKMHZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCKMHZ` reader - Data Flash Access Frequency Register"]
pub type FCKMHZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCKMHZ` writer - Data Flash Access Frequency Register"]
pub type FCKMHZ_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FCKMHZ_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data Flash Access Frequency Register"]
    #[inline(always)]
    pub fn fckmhz(&self) -> FCKMHZ_R {
        FCKMHZ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Flash Access Frequency Register"]
    #[inline(always)]
    #[must_use]
    pub fn fckmhz(&mut self) -> FCKMHZ_W<0> {
        FCKMHZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Flash Access Frequency Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fckmhz](index.html) module"]
pub struct FCKMHZ_SPEC;
impl crate::RegisterSpec for FCKMHZ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fckmhz::R](R) reader structure"]
impl crate::Readable for FCKMHZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fckmhz::W](W) writer structure"]
impl crate::Writable for FCKMHZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCKMHZ to value 0x3c"]
impl crate::Resettable for FCKMHZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x3c;
}
