#[doc = "Register `IPGR` reader"]
pub struct R(crate::R<IPGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPGR` writer"]
pub struct W(crate::W<IPGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPGR_SPEC>;
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
impl From<crate::W<IPGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPG` reader - Interpacket Gap Range:\"16bit time(0x00)\"-\"140bit time(0x1F)\""]
pub type IPG_R = crate::FieldReader<u8, IPG_A>;
#[doc = "Interpacket Gap Range:\"16bit time(0x00)\"-\"140bit time(0x1F)\"\n\nValue on reset: 20"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IPG_A {
    #[doc = "20: 96 bit time (initial value)"]
    _14H = 20,
}
impl From<IPG_A> for u8 {
    #[inline(always)]
    fn from(variant: IPG_A) -> Self {
        variant as _
    }
}
impl IPG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IPG_A> {
        match self.bits {
            20 => Some(IPG_A::_14H),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_14H`"]
    #[inline(always)]
    pub fn is_14h(&self) -> bool {
        *self == IPG_A::_14H
    }
}
#[doc = "Field `IPG` writer - Interpacket Gap Range:\"16bit time(0x00)\"-\"140bit time(0x1F)\""]
pub type IPG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPGR_SPEC, u8, IPG_A, 5, O>;
impl<'a, const O: u8> IPG_W<'a, O> {
    #[doc = "96 bit time (initial value)"]
    #[inline(always)]
    pub fn _14h(self) -> &'a mut W {
        self.variant(IPG_A::_14H)
    }
}
impl R {
    #[doc = "Bits 0:4 - Interpacket Gap Range:\"16bit time(0x00)\"-\"140bit time(0x1F)\""]
    #[inline(always)]
    pub fn ipg(&self) -> IPG_R {
        IPG_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Interpacket Gap Range:\"16bit time(0x00)\"-\"140bit time(0x1F)\""]
    #[inline(always)]
    #[must_use]
    pub fn ipg(&mut self) -> IPG_W<0> {
        IPG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPG Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipgr](index.html) module"]
pub struct IPGR_SPEC;
impl crate::RegisterSpec for IPGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipgr::R](R) reader structure"]
impl crate::Readable for IPGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipgr::W](W) writer structure"]
impl crate::Writable for IPGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPGR to value 0x14"]
impl crate::Resettable for IPGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
