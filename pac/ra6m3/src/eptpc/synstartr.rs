#[doc = "Register `SYNSTARTR` reader"]
pub struct R(crate::R<SYNSTARTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNSTARTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNSTARTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNSTARTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNSTARTR` writer"]
pub struct W(crate::W<SYNSTARTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNSTARTR_SPEC>;
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
impl From<crate::W<SYNSTARTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNSTARTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STR` reader - Slave Time Synchronization Control"]
pub type STR_R = crate::BitReader<STR_A>;
#[doc = "Slave Time Synchronization Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STR_A {
    #[doc = "0: Slave time synchronization is stopped."]
    _0 = 0,
    #[doc = "1: Slave time synchronization is started."]
    _1 = 1,
}
impl From<STR_A> for bool {
    #[inline(always)]
    fn from(variant: STR_A) -> Self {
        variant as u8 != 0
    }
}
impl STR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STR_A {
        match self.bits {
            false => STR_A::_0,
            true => STR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STR_A::_1
    }
}
#[doc = "Field `STR` writer - Slave Time Synchronization Control"]
pub type STR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNSTARTR_SPEC, STR_A, O>;
impl<'a, const O: u8> STR_W<'a, O> {
    #[doc = "Slave time synchronization is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STR_A::_0)
    }
    #[doc = "Slave time synchronization is started."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Slave Time Synchronization Control"]
    #[inline(always)]
    pub fn str(&self) -> STR_R {
        STR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Time Synchronization Control"]
    #[inline(always)]
    #[must_use]
    pub fn str(&mut self) -> STR_W<0> {
        STR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Time Synchronization Start Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [synstartr](index.html) module"]
pub struct SYNSTARTR_SPEC;
impl crate::RegisterSpec for SYNSTARTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [synstartr::R](R) reader structure"]
impl crate::Readable for SYNSTARTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [synstartr::W](W) writer structure"]
impl crate::Writable for SYNSTARTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNSTARTR to value 0"]
impl crate::Resettable for SYNSTARTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
