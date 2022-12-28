#[doc = "Register `VBTBER` reader"]
pub struct R(crate::R<VBTBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBTBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBTBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBTBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VBTBER` writer"]
pub struct W(crate::W<VBTBER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VBTBER_SPEC>;
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
impl From<crate::W<VBTBER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VBTBER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBAE` reader - VBATT backup register access enable bit"]
pub type VBAE_R = crate::BitReader<VBAE_A>;
#[doc = "VBATT backup register access enable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBAE_A {
    #[doc = "0: Disable to access VBTBKR"]
    _0 = 0,
    #[doc = "1: Enable to access VBTBKR"]
    _1 = 1,
}
impl From<VBAE_A> for bool {
    #[inline(always)]
    fn from(variant: VBAE_A) -> Self {
        variant as u8 != 0
    }
}
impl VBAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBAE_A {
        match self.bits {
            false => VBAE_A::_0,
            true => VBAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBAE_A::_1
    }
}
#[doc = "Field `VBAE` writer - VBATT backup register access enable bit"]
pub type VBAE_W<'a, const O: u8> = crate::BitWriter<'a, u8, VBTBER_SPEC, VBAE_A, O>;
impl<'a, const O: u8> VBAE_W<'a, O> {
    #[doc = "Disable to access VBTBKR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBAE_A::_0)
    }
    #[doc = "Enable to access VBTBKR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBAE_A::_1)
    }
}
impl R {
    #[doc = "Bit 3 - VBATT backup register access enable bit"]
    #[inline(always)]
    pub fn vbae(&self) -> VBAE_R {
        VBAE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - VBATT backup register access enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn vbae(&mut self) -> VBAE_W<3> {
        VBAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VBATT Backup Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbtber](index.html) module"]
pub struct VBTBER_SPEC;
impl crate::RegisterSpec for VBTBER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vbtber::R](R) reader structure"]
impl crate::Readable for VBTBER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vbtber::W](W) writer structure"]
impl crate::Writable for VBTBER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTBER to value 0x08"]
impl crate::Resettable for VBTBER_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
