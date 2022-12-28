#[doc = "Register `BCTL` reader"]
pub struct R(crate::R<BCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCTL` writer"]
pub struct W(crate::W<BCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCTL_SPEC>;
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
impl From<crate::W<BCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSE` reader - Bus Enable"]
pub type BUSE_R = crate::BitReader<BUSE_A>;
#[doc = "Bus Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSE_A {
    #[doc = "0: IIC bus operation is disabled."]
    _0 = 0,
    #[doc = "1: IIC bus operation is enabled."]
    _1 = 1,
}
impl From<BUSE_A> for bool {
    #[inline(always)]
    fn from(variant: BUSE_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSE_A {
        match self.bits {
            false => BUSE_A::_0,
            true => BUSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSE_A::_1
    }
}
#[doc = "Field `BUSE` writer - Bus Enable"]
pub type BUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCTL_SPEC, BUSE_A, O>;
impl<'a, const O: u8> BUSE_W<'a, O> {
    #[doc = "IIC bus operation is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSE_A::_0)
    }
    #[doc = "IIC bus operation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSE_A::_1)
    }
}
impl R {
    #[doc = "Bit 31 - Bus Enable"]
    #[inline(always)]
    pub fn buse(&self) -> BUSE_R {
        BUSE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Bus Enable"]
    #[inline(always)]
    #[must_use]
    pub fn buse(&mut self) -> BUSE_W<31> {
        BUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bctl](index.html) module"]
pub struct BCTL_SPEC;
impl crate::RegisterSpec for BCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bctl::R](R) reader structure"]
impl crate::Readable for BCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bctl::W](W) writer structure"]
impl crate::Writable for BCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCTL to value 0"]
impl crate::Resettable for BCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
