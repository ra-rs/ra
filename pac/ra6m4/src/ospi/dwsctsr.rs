#[doc = "Register `DWSCTSR` reader"]
pub struct R(crate::R<DWSCTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DWSCTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DWSCTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DWSCTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DWSCTSR` writer"]
pub struct W(crate::W<DWSCTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DWSCTSR_SPEC>;
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
impl From<crate::W<DWSCTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DWSCTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSN0` reader - Indicates the number of bytes to translate in single continuous write of device 0."]
pub type CTSN0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CTSN0` writer - Indicates the number of bytes to translate in single continuous write of device 0."]
pub type CTSN0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DWSCTSR_SPEC, u16, u16, 11, O>;
#[doc = "Field `CTSN1` reader - Indicates the number of bytes to translate in single continuous write of device 1."]
pub type CTSN1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CTSN1` writer - Indicates the number of bytes to translate in single continuous write of device 1."]
pub type CTSN1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DWSCTSR_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Indicates the number of bytes to translate in single continuous write of device 0."]
    #[inline(always)]
    pub fn ctsn0(&self) -> CTSN0_R {
        CTSN0_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Indicates the number of bytes to translate in single continuous write of device 1."]
    #[inline(always)]
    pub fn ctsn1(&self) -> CTSN1_R {
        CTSN1_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Indicates the number of bytes to translate in single continuous write of device 0."]
    #[inline(always)]
    #[must_use]
    pub fn ctsn0(&mut self) -> CTSN0_W<0> {
        CTSN0_W::new(self)
    }
    #[doc = "Bits 16:26 - Indicates the number of bytes to translate in single continuous write of device 1."]
    #[inline(always)]
    #[must_use]
    pub fn ctsn1(&mut self) -> CTSN1_W<16> {
        CTSN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Memory Map Write single continuous translating size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dwsctsr](index.html) module"]
pub struct DWSCTSR_SPEC;
impl crate::RegisterSpec for DWSCTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dwsctsr::R](R) reader structure"]
impl crate::Readable for DWSCTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dwsctsr::W](W) writer structure"]
impl crate::Writable for DWSCTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DWSCTSR to value 0"]
impl crate::Resettable for DWSCTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
