#[doc = "Register `GETINFOR` reader"]
pub struct R(crate::R<GETINFOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GETINFOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GETINFOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GETINFOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GETINFOR` writer"]
pub struct W(crate::W<GETINFOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GETINFOR_SPEC>;
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
impl From<crate::W<GETINFOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GETINFOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INFO` reader - Information Retention ControlNOTE: Once information fetching is directed, values of various statistical information read before completion of information fetching are not guaranteed."]
pub type INFO_R = crate::BitReader<INFO_A>;
#[doc = "Information Retention ControlNOTE: Once information fetching is directed, values of various statistical information read before completion of information fetching are not guaranteed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INFO_A {
    #[doc = "0: Has no effects.(write) / Information retention is completed.(read)"]
    _0 = 0,
    #[doc = "1: Information is retained.(write) / Processing for information retention is in progress.(read)"]
    _1 = 1,
}
impl From<INFO_A> for bool {
    #[inline(always)]
    fn from(variant: INFO_A) -> Self {
        variant as u8 != 0
    }
}
impl INFO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INFO_A {
        match self.bits {
            false => INFO_A::_0,
            true => INFO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INFO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INFO_A::_1
    }
}
#[doc = "Field `INFO` writer - Information Retention ControlNOTE: Once information fetching is directed, values of various statistical information read before completion of information fetching are not guaranteed."]
pub type INFO_W<'a, const O: u8> = crate::BitWriter<'a, u32, GETINFOR_SPEC, INFO_A, O>;
impl<'a, const O: u8> INFO_W<'a, O> {
    #[doc = "Has no effects.(write) / Information retention is completed.(read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INFO_A::_0)
    }
    #[doc = "Information is retained.(write) / Processing for information retention is in progress.(read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INFO_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Information Retention ControlNOTE: Once information fetching is directed, values of various statistical information read before completion of information fetching are not guaranteed."]
    #[inline(always)]
    pub fn info(&self) -> INFO_R {
        INFO_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Information Retention ControlNOTE: Once information fetching is directed, values of various statistical information read before completion of information fetching are not guaranteed."]
    #[inline(always)]
    #[must_use]
    pub fn info(&mut self) -> INFO_W<0> {
        INFO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Statistical Information Retention Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [getinfor](index.html) module"]
pub struct GETINFOR_SPEC;
impl crate::RegisterSpec for GETINFOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [getinfor::R](R) reader structure"]
impl crate::Readable for GETINFOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [getinfor::W](W) writer structure"]
impl crate::Writable for GETINFOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GETINFOR to value 0"]
impl crate::Resettable for GETINFOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
