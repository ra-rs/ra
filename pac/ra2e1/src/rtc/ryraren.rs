#[doc = "Register `RYRAREN` reader"]
pub struct R(crate::R<RYRAREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RYRAREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RYRAREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RYRAREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RYRAREN` writer"]
pub struct W(crate::W<RYRAREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RYRAREN_SPEC>;
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
impl From<crate::W<RYRAREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RYRAREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENB` reader - ENB"]
pub type ENB_R = crate::BitReader<ENB_A>;
#[doc = "ENB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENB_A {
    #[doc = "0: Do not compare register value with the RYRCNT counter value"]
    _0 = 0,
    #[doc = "1: Compare register value with the RYRCNT counter value"]
    _1 = 1,
}
impl From<ENB_A> for bool {
    #[inline(always)]
    fn from(variant: ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB_A {
        match self.bits {
            false => ENB_A::_0,
            true => ENB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENB_A::_1
    }
}
#[doc = "Field `ENB` writer - ENB"]
pub type ENB_W<'a, const O: u8> = crate::BitWriter<'a, u8, RYRAREN_SPEC, ENB_A, O>;
impl<'a, const O: u8> ENB_W<'a, O> {
    #[doc = "Do not compare register value with the RYRCNT counter value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENB_A::_0)
    }
    #[doc = "Compare register value with the RYRCNT counter value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENB_A::_1)
    }
}
impl R {
    #[doc = "Bit 7 - ENB"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - ENB"]
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W<7> {
        ENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Year Alarm Enable Register (in Calendar Count Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ryraren](index.html) module"]
pub struct RYRAREN_SPEC;
impl crate::RegisterSpec for RYRAREN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ryraren::R](R) reader structure"]
impl crate::Readable for RYRAREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ryraren::W](W) writer structure"]
impl crate::Writable for RYRAREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RYRAREN to value 0"]
impl crate::Resettable for RYRAREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
