#[doc = "Register `CF1CR` reader"]
pub struct R(crate::R<CF1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CF1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CF1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CF1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CF1CR` writer"]
pub struct W(crate::W<CF1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CF1CR_SPEC>;
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
impl From<crate::W<CF1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CF1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF1CE0` reader - Control Field 1 Bit 0 Compare Enable"]
pub type CF1CE0_R = crate::BitReader<CF1CE0_A>;
#[doc = "Control Field 1 Bit 0 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF1CE0_A {
    #[doc = "0: Comparison with bit 0 of Control Field 1 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 0 of Control Field 1 is enabled."]
    _1 = 1,
}
impl From<CF1CE0_A> for bool {
    #[inline(always)]
    fn from(variant: CF1CE0_A) -> Self {
        variant as u8 != 0
    }
}
impl CF1CE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF1CE0_A {
        match self.bits {
            false => CF1CE0_A::_0,
            true => CF1CE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF1CE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF1CE0_A::_1
    }
}
#[doc = "Field `CF1CE0` writer - Control Field 1 Bit 0 Compare Enable"]
pub type CF1CE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF1CR_SPEC, CF1CE0_A, O>;
impl<'a, const O: u8> CF1CE0_W<'a, O> {
    #[doc = "Comparison with bit 0 of Control Field 1 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF1CE0_A::_0)
    }
    #[doc = "Comparison with bit 0 of Control Field 1 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF1CE0_A::_1)
    }
}
#[doc = "Field `CF1CE1` reader - Control Field 1 Bit 1 Compare Enable"]
pub type CF1CE1_R = crate::BitReader<CF1CE1_A>;
#[doc = "Control Field 1 Bit 1 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF1CE1_A {
    #[doc = "0: Comparison with bit 1 of Control Field 1 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 1 of Control Field 1 is enabled."]
    _1 = 1,
}
impl From<CF1CE1_A> for bool {
    #[inline(always)]
    fn from(variant: CF1CE1_A) -> Self {
        variant as u8 != 0
    }
}
impl CF1CE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF1CE1_A {
        match self.bits {
            false => CF1CE1_A::_0,
            true => CF1CE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF1CE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF1CE1_A::_1
    }
}
#[doc = "Field `CF1CE1` writer - Control Field 1 Bit 1 Compare Enable"]
pub type CF1CE1_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF1CR_SPEC, CF1CE1_A, O>;
impl<'a, const O: u8> CF1CE1_W<'a, O> {
    #[doc = "Comparison with bit 1 of Control Field 1 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF1CE1_A::_0)
    }
    #[doc = "Comparison with bit 1 of Control Field 1 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF1CE1_A::_1)
    }
}
#[doc = "Field `CF1CE2` reader - Control Field 1 Bit 2 Compare Enable"]
pub type CF1CE2_R = crate::BitReader<CF1CE2_A>;
#[doc = "Control Field 1 Bit 2 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF1CE2_A {
    #[doc = "0: Comparison with bit 2 of Control Field 1 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 2 of Control Field 1 is enabled."]
    _1 = 1,
}
impl From<CF1CE2_A> for bool {
    #[inline(always)]
    fn from(variant: CF1CE2_A) -> Self {
        variant as u8 != 0
    }
}
impl CF1CE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF1CE2_A {
        match self.bits {
            false => CF1CE2_A::_0,
            true => CF1CE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF1CE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF1CE2_A::_1
    }
}
#[doc = "Field `CF1CE2` writer - Control Field 1 Bit 2 Compare Enable"]
pub type CF1CE2_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF1CR_SPEC, CF1CE2_A, O>;
impl<'a, const O: u8> CF1CE2_W<'a, O> {
    #[doc = "Comparison with bit 2 of Control Field 1 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF1CE2_A::_0)
    }
    #[doc = "Comparison with bit 2 of Control Field 1 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF1CE2_A::_1)
    }
}
#[doc = "Field `CF1CE3` reader - Control Field 1 Bit 3 Compare Enable"]
pub type CF1CE3_R = crate::BitReader<CF1CE3_A>;
#[doc = "Control Field 1 Bit 3 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF1CE3_A {
    #[doc = "0: Comparison with bit 3 of Control Field 1 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 3 of Control Field 1 is enabled."]
    _1 = 1,
}
impl From<CF1CE3_A> for bool {
    #[inline(always)]
    fn from(variant: CF1CE3_A) -> Self {
        variant as u8 != 0
    }
}
impl CF1CE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF1CE3_A {
        match self.bits {
            false => CF1CE3_A::_0,
            true => CF1CE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF1CE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF1CE3_A::_1
    }
}
#[doc = "Field `CF1CE3` writer - Control Field 1 Bit 3 Compare Enable"]
pub type CF1CE3_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF1CR_SPEC, CF1CE3_A, O>;
impl<'a, const O: u8> CF1CE3_W<'a, O> {
    #[doc = "Comparison with bit 3 of Control Field 1 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF1CE3_A::_0)
    }
    #[doc = "Comparison with bit 3 of Control Field 1 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF1CE3_A::_1)
    }
}
#[doc = "Field `CF1CE4` reader - Control Field 1 Bit 4 Compare Enable"]
pub type CF1CE4_R = crate::BitReader<CF1CE4_A>;
#[doc = "Control Field 1 Bit 4 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF1CE4_A {
    #[doc = "0: Comparison with bit 4 of Control Field 1 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 4 of Control Field 1 is enabled."]
    _1 = 1,
}
impl From<CF1CE4_A> for bool {
    #[inline(always)]
    fn from(variant: CF1CE4_A) -> Self {
        variant as u8 != 0
    }
}
impl CF1CE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF1CE4_A {
        match self.bits {
            false => CF1CE4_A::_0,
            true => CF1CE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF1CE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF1CE4_A::_1
    }
}
#[doc = "Field `CF1CE4` writer - Control Field 1 Bit 4 Compare Enable"]
pub type CF1CE4_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF1CR_SPEC, CF1CE4_A, O>;
impl<'a, const O: u8> CF1CE4_W<'a, O> {
    #[doc = "Comparison with bit 4 of Control Field 1 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF1CE4_A::_0)
    }
    #[doc = "Comparison with bit 4 of Control Field 1 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF1CE4_A::_1)
    }
}
#[doc = "Field `CF1CE5` reader - Control Field 1 Bit 5 Compare Enable"]
pub type CF1CE5_R = crate::BitReader<CF1CE5_A>;
#[doc = "Control Field 1 Bit 5 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF1CE5_A {
    #[doc = "0: Comparison with bit 5 of Control Field 1 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 5 of Control Field 1 is enabled."]
    _1 = 1,
}
impl From<CF1CE5_A> for bool {
    #[inline(always)]
    fn from(variant: CF1CE5_A) -> Self {
        variant as u8 != 0
    }
}
impl CF1CE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF1CE5_A {
        match self.bits {
            false => CF1CE5_A::_0,
            true => CF1CE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF1CE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF1CE5_A::_1
    }
}
#[doc = "Field `CF1CE5` writer - Control Field 1 Bit 5 Compare Enable"]
pub type CF1CE5_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF1CR_SPEC, CF1CE5_A, O>;
impl<'a, const O: u8> CF1CE5_W<'a, O> {
    #[doc = "Comparison with bit 5 of Control Field 1 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF1CE5_A::_0)
    }
    #[doc = "Comparison with bit 5 of Control Field 1 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF1CE5_A::_1)
    }
}
#[doc = "Field `CF1CE6` reader - Control Field 1 Bit 6 Compare Enable"]
pub type CF1CE6_R = crate::BitReader<CF1CE6_A>;
#[doc = "Control Field 1 Bit 6 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF1CE6_A {
    #[doc = "0: Comparison with bit 6 of Control Field 1 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 6 of Control Field 1 is enabled."]
    _1 = 1,
}
impl From<CF1CE6_A> for bool {
    #[inline(always)]
    fn from(variant: CF1CE6_A) -> Self {
        variant as u8 != 0
    }
}
impl CF1CE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF1CE6_A {
        match self.bits {
            false => CF1CE6_A::_0,
            true => CF1CE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF1CE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF1CE6_A::_1
    }
}
#[doc = "Field `CF1CE6` writer - Control Field 1 Bit 6 Compare Enable"]
pub type CF1CE6_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF1CR_SPEC, CF1CE6_A, O>;
impl<'a, const O: u8> CF1CE6_W<'a, O> {
    #[doc = "Comparison with bit 6 of Control Field 1 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF1CE6_A::_0)
    }
    #[doc = "Comparison with bit 6 of Control Field 1 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF1CE6_A::_1)
    }
}
#[doc = "Field `CF1CE7` reader - Control Field 1 Bit 7 Compare Enable"]
pub type CF1CE7_R = crate::BitReader<CF1CE7_A>;
#[doc = "Control Field 1 Bit 7 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF1CE7_A {
    #[doc = "0: Comparison with bit 7 of Control Field 1 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 7 of Control Field 1 is enabled."]
    _1 = 1,
}
impl From<CF1CE7_A> for bool {
    #[inline(always)]
    fn from(variant: CF1CE7_A) -> Self {
        variant as u8 != 0
    }
}
impl CF1CE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF1CE7_A {
        match self.bits {
            false => CF1CE7_A::_0,
            true => CF1CE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF1CE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF1CE7_A::_1
    }
}
#[doc = "Field `CF1CE7` writer - Control Field 1 Bit 7 Compare Enable"]
pub type CF1CE7_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF1CR_SPEC, CF1CE7_A, O>;
impl<'a, const O: u8> CF1CE7_W<'a, O> {
    #[doc = "Comparison with bit 7 of Control Field 1 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF1CE7_A::_0)
    }
    #[doc = "Comparison with bit 7 of Control Field 1 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF1CE7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Control Field 1 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce0(&self) -> CF1CE0_R {
        CF1CE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control Field 1 Bit 1 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce1(&self) -> CF1CE1_R {
        CF1CE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control Field 1 Bit 2 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce2(&self) -> CF1CE2_R {
        CF1CE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control Field 1 Bit 3 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce3(&self) -> CF1CE3_R {
        CF1CE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control Field 1 Bit 4 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce4(&self) -> CF1CE4_R {
        CF1CE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Control Field 1 Bit 5 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce5(&self) -> CF1CE5_R {
        CF1CE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Control Field 1 Bit 6 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce6(&self) -> CF1CE6_R {
        CF1CE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Control Field 1 Bit 7 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce7(&self) -> CF1CE7_R {
        CF1CE7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control Field 1 Bit 0 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf1ce0(&mut self) -> CF1CE0_W<0> {
        CF1CE0_W::new(self)
    }
    #[doc = "Bit 1 - Control Field 1 Bit 1 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf1ce1(&mut self) -> CF1CE1_W<1> {
        CF1CE1_W::new(self)
    }
    #[doc = "Bit 2 - Control Field 1 Bit 2 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf1ce2(&mut self) -> CF1CE2_W<2> {
        CF1CE2_W::new(self)
    }
    #[doc = "Bit 3 - Control Field 1 Bit 3 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf1ce3(&mut self) -> CF1CE3_W<3> {
        CF1CE3_W::new(self)
    }
    #[doc = "Bit 4 - Control Field 1 Bit 4 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf1ce4(&mut self) -> CF1CE4_W<4> {
        CF1CE4_W::new(self)
    }
    #[doc = "Bit 5 - Control Field 1 Bit 5 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf1ce5(&mut self) -> CF1CE5_W<5> {
        CF1CE5_W::new(self)
    }
    #[doc = "Bit 6 - Control Field 1 Bit 6 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf1ce6(&mut self) -> CF1CE6_W<6> {
        CF1CE6_W::new(self)
    }
    #[doc = "Bit 7 - Control Field 1 Bit 7 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf1ce7(&mut self) -> CF1CE7_W<7> {
        CF1CE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Field 1 Compare Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cf1cr](index.html) module"]
pub struct CF1CR_SPEC;
impl crate::RegisterSpec for CF1CR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cf1cr::R](R) reader structure"]
impl crate::Readable for CF1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cf1cr::W](W) writer structure"]
impl crate::Writable for CF1CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CF1CR to value 0"]
impl crate::Resettable for CF1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
