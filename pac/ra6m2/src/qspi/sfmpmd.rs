#[doc = "Register `SFMPMD` reader"]
pub struct R(crate::R<SFMPMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMPMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMPMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMPMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMPMD` writer"]
pub struct W(crate::W<SFMPMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMPMD_SPEC>;
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
impl From<crate::W<SFMPMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMPMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFMWPL` reader - Specify level of WP pin"]
pub type SFMWPL_R = crate::BitReader<SFMWPL_A>;
#[doc = "Specify level of WP pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMWPL_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<SFMWPL_A> for bool {
    #[inline(always)]
    fn from(variant: SFMWPL_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMWPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMWPL_A {
        match self.bits {
            false => SFMWPL_A::_0,
            true => SFMWPL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMWPL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMWPL_A::_1
    }
}
#[doc = "Field `SFMWPL` writer - Specify level of WP pin"]
pub type SFMWPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMPMD_SPEC, SFMWPL_A, O>;
impl<'a, const O: u8> SFMWPL_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMWPL_A::_0)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMWPL_A::_1)
    }
}
impl R {
    #[doc = "Bit 2 - Specify level of WP pin"]
    #[inline(always)]
    pub fn sfmwpl(&self) -> SFMWPL_R {
        SFMWPL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Specify level of WP pin"]
    #[inline(always)]
    #[must_use]
    pub fn sfmwpl(&mut self) -> SFMWPL_W<2> {
        SFMWPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmpmd](index.html) module"]
pub struct SFMPMD_SPEC;
impl crate::RegisterSpec for SFMPMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmpmd::R](R) reader structure"]
impl crate::Readable for SFMPMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmpmd::W](W) writer structure"]
impl crate::Writable for SFMPMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMPMD to value 0"]
impl crate::Resettable for SFMPMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
