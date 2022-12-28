#[doc = "Register `INCTL` reader"]
pub struct R(crate::R<INCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INCTL` writer"]
pub struct W(crate::W<INCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INCTL_SPEC>;
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
impl From<crate::W<INCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DNFS` reader - Digital Noise Filter Stage Selection"]
pub type DNFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DNFS` writer - Digital Noise Filter Stage Selection"]
pub type DNFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `DNFE` reader - Digital Noise Filter Circuit Enable"]
pub type DNFE_R = crate::BitReader<DNFE_A>;
#[doc = "Digital Noise Filter Circuit Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNFE_A {
    #[doc = "0: No digital noise filter circuit is used."]
    _0 = 0,
    #[doc = "1: A digital noise filter circuit is used."]
    _1 = 1,
}
impl From<DNFE_A> for bool {
    #[inline(always)]
    fn from(variant: DNFE_A) -> Self {
        variant as u8 != 0
    }
}
impl DNFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNFE_A {
        match self.bits {
            false => DNFE_A::_0,
            true => DNFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DNFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DNFE_A::_1
    }
}
#[doc = "Field `DNFE` writer - Digital Noise Filter Circuit Enable"]
pub type DNFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INCTL_SPEC, DNFE_A, O>;
impl<'a, const O: u8> DNFE_W<'a, O> {
    #[doc = "No digital noise filter circuit is used."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DNFE_A::_0)
    }
    #[doc = "A digital noise filter circuit is used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DNFE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Digital Noise Filter Stage Selection"]
    #[inline(always)]
    pub fn dnfs(&self) -> DNFS_R {
        DNFS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Digital Noise Filter Circuit Enable"]
    #[inline(always)]
    pub fn dnfe(&self) -> DNFE_R {
        DNFE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Digital Noise Filter Stage Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dnfs(&mut self) -> DNFS_W<0> {
        DNFS_W::new(self)
    }
    #[doc = "Bit 4 - Digital Noise Filter Circuit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dnfe(&mut self) -> DNFE_W<4> {
        DNFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inctl](index.html) module"]
pub struct INCTL_SPEC;
impl crate::RegisterSpec for INCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inctl::R](R) reader structure"]
impl crate::Readable for INCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inctl::W](W) writer structure"]
impl crate::Writable for INCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INCTL to value 0xd0"]
impl crate::Resettable for INCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xd0;
}
