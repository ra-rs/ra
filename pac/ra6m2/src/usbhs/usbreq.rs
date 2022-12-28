#[doc = "Register `USBREQ` reader"]
pub struct R(crate::R<USBREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBREQ` writer"]
pub struct W(crate::W<USBREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBREQ_SPEC>;
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
impl From<crate::W<USBREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BMREQUESTTYPE` reader - USB request bmRequestType value Finction controller selected : read-only Host controller selected : read-write"]
pub type BMREQUESTTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BMREQUESTTYPE` writer - USB request bmRequestType value Finction controller selected : read-only Host controller selected : read-write"]
pub type BMREQUESTTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, USBREQ_SPEC, u8, u8, 8, O>;
#[doc = "Field `BREQUEST` reader - USB request bRequest value Finction controller selected : read-only Host controller selected : read-write"]
pub type BREQUEST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BREQUEST` writer - USB request bRequest value Finction controller selected : read-only Host controller selected : read-write"]
pub type BREQUEST_W<'a, const O: u8> = crate::FieldWriter<'a, u16, USBREQ_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - USB request bmRequestType value Finction controller selected : read-only Host controller selected : read-write"]
    #[inline(always)]
    pub fn bmrequesttype(&self) -> BMREQUESTTYPE_R {
        BMREQUESTTYPE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - USB request bRequest value Finction controller selected : read-only Host controller selected : read-write"]
    #[inline(always)]
    pub fn brequest(&self) -> BREQUEST_R {
        BREQUEST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USB request bmRequestType value Finction controller selected : read-only Host controller selected : read-write"]
    #[inline(always)]
    #[must_use]
    pub fn bmrequesttype(&mut self) -> BMREQUESTTYPE_W<0> {
        BMREQUESTTYPE_W::new(self)
    }
    #[doc = "Bits 8:15 - USB request bRequest value Finction controller selected : read-only Host controller selected : read-write"]
    #[inline(always)]
    #[must_use]
    pub fn brequest(&mut self) -> BREQUEST_W<8> {
        BREQUEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Request Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbreq](index.html) module"]
pub struct USBREQ_SPEC;
impl crate::RegisterSpec for USBREQ_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbreq::R](R) reader structure"]
impl crate::Readable for USBREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbreq::W](W) writer structure"]
impl crate::Writable for USBREQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBREQ to value 0"]
impl crate::Resettable for USBREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
