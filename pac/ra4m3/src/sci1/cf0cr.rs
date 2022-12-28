#[doc = "Register `CF0CR` reader"]
pub struct R(crate::R<CF0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CF0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CF0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CF0CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CF0CR` writer"]
pub struct W(crate::W<CF0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CF0CR_SPEC>;
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
impl From<crate::W<CF0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CF0CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF0CE0` reader - Control Field 0 Bit 0 Compare Enable"]
pub type CF0CE0_R = crate::BitReader<CF0CE0_A>;
#[doc = "Control Field 0 Bit 0 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF0CE0_A {
    #[doc = "0: Comparison with bit 0 of Control Field 0 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 0 of Control Field 0 is enabled."]
    _1 = 1,
}
impl From<CF0CE0_A> for bool {
    #[inline(always)]
    fn from(variant: CF0CE0_A) -> Self {
        variant as u8 != 0
    }
}
impl CF0CE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF0CE0_A {
        match self.bits {
            false => CF0CE0_A::_0,
            true => CF0CE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF0CE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF0CE0_A::_1
    }
}
#[doc = "Field `CF0CE0` writer - Control Field 0 Bit 0 Compare Enable"]
pub type CF0CE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF0CR_SPEC, CF0CE0_A, O>;
impl<'a, const O: u8> CF0CE0_W<'a, O> {
    #[doc = "Comparison with bit 0 of Control Field 0 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF0CE0_A::_0)
    }
    #[doc = "Comparison with bit 0 of Control Field 0 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF0CE0_A::_1)
    }
}
#[doc = "Field `CF0CE1` reader - Control Field 1 Bit 0 Compare Enable"]
pub type CF0CE1_R = crate::BitReader<CF0CE1_A>;
#[doc = "Control Field 1 Bit 0 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF0CE1_A {
    #[doc = "0: Comparison with bit 1 of Control Field 0 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 1 of Control Field 0 is enabled."]
    _1 = 1,
}
impl From<CF0CE1_A> for bool {
    #[inline(always)]
    fn from(variant: CF0CE1_A) -> Self {
        variant as u8 != 0
    }
}
impl CF0CE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF0CE1_A {
        match self.bits {
            false => CF0CE1_A::_0,
            true => CF0CE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF0CE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF0CE1_A::_1
    }
}
#[doc = "Field `CF0CE1` writer - Control Field 1 Bit 0 Compare Enable"]
pub type CF0CE1_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF0CR_SPEC, CF0CE1_A, O>;
impl<'a, const O: u8> CF0CE1_W<'a, O> {
    #[doc = "Comparison with bit 1 of Control Field 0 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF0CE1_A::_0)
    }
    #[doc = "Comparison with bit 1 of Control Field 0 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF0CE1_A::_1)
    }
}
#[doc = "Field `CF0CE2` reader - Control Field 2 Bit 0 Compare Enable"]
pub type CF0CE2_R = crate::BitReader<CF0CE2_A>;
#[doc = "Control Field 2 Bit 0 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF0CE2_A {
    #[doc = "0: Comparison with bit 2 of Control Field 0 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 2 of Control Field 0 is enabled."]
    _1 = 1,
}
impl From<CF0CE2_A> for bool {
    #[inline(always)]
    fn from(variant: CF0CE2_A) -> Self {
        variant as u8 != 0
    }
}
impl CF0CE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF0CE2_A {
        match self.bits {
            false => CF0CE2_A::_0,
            true => CF0CE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF0CE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF0CE2_A::_1
    }
}
#[doc = "Field `CF0CE2` writer - Control Field 2 Bit 0 Compare Enable"]
pub type CF0CE2_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF0CR_SPEC, CF0CE2_A, O>;
impl<'a, const O: u8> CF0CE2_W<'a, O> {
    #[doc = "Comparison with bit 2 of Control Field 0 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF0CE2_A::_0)
    }
    #[doc = "Comparison with bit 2 of Control Field 0 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF0CE2_A::_1)
    }
}
#[doc = "Field `CF0CE3` reader - Control Field 3 Bit 0 Compare Enable"]
pub type CF0CE3_R = crate::BitReader<CF0CE3_A>;
#[doc = "Control Field 3 Bit 0 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF0CE3_A {
    #[doc = "0: Comparison with bit 3 of Control Field 0 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 3 of Control Field 0 is enabled."]
    _1 = 1,
}
impl From<CF0CE3_A> for bool {
    #[inline(always)]
    fn from(variant: CF0CE3_A) -> Self {
        variant as u8 != 0
    }
}
impl CF0CE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF0CE3_A {
        match self.bits {
            false => CF0CE3_A::_0,
            true => CF0CE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF0CE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF0CE3_A::_1
    }
}
#[doc = "Field `CF0CE3` writer - Control Field 3 Bit 0 Compare Enable"]
pub type CF0CE3_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF0CR_SPEC, CF0CE3_A, O>;
impl<'a, const O: u8> CF0CE3_W<'a, O> {
    #[doc = "Comparison with bit 3 of Control Field 0 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF0CE3_A::_0)
    }
    #[doc = "Comparison with bit 3 of Control Field 0 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF0CE3_A::_1)
    }
}
#[doc = "Field `CF0CE4` reader - Control Field 4 Bit 0 Compare Enable"]
pub type CF0CE4_R = crate::BitReader<CF0CE4_A>;
#[doc = "Control Field 4 Bit 0 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF0CE4_A {
    #[doc = "0: Comparison with bit 4 of Control Field 0 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 4 of Control Field 0 is enabled."]
    _1 = 1,
}
impl From<CF0CE4_A> for bool {
    #[inline(always)]
    fn from(variant: CF0CE4_A) -> Self {
        variant as u8 != 0
    }
}
impl CF0CE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF0CE4_A {
        match self.bits {
            false => CF0CE4_A::_0,
            true => CF0CE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF0CE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF0CE4_A::_1
    }
}
#[doc = "Field `CF0CE4` writer - Control Field 4 Bit 0 Compare Enable"]
pub type CF0CE4_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF0CR_SPEC, CF0CE4_A, O>;
impl<'a, const O: u8> CF0CE4_W<'a, O> {
    #[doc = "Comparison with bit 4 of Control Field 0 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF0CE4_A::_0)
    }
    #[doc = "Comparison with bit 4 of Control Field 0 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF0CE4_A::_1)
    }
}
#[doc = "Field `CF0CE5` reader - Control Field 5 Bit 0 Compare Enable"]
pub type CF0CE5_R = crate::BitReader<CF0CE5_A>;
#[doc = "Control Field 5 Bit 0 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF0CE5_A {
    #[doc = "0: Comparison with bit 5 of Control Field 0 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 5 of Control Field 0 is enabled."]
    _1 = 1,
}
impl From<CF0CE5_A> for bool {
    #[inline(always)]
    fn from(variant: CF0CE5_A) -> Self {
        variant as u8 != 0
    }
}
impl CF0CE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF0CE5_A {
        match self.bits {
            false => CF0CE5_A::_0,
            true => CF0CE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF0CE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF0CE5_A::_1
    }
}
#[doc = "Field `CF0CE5` writer - Control Field 5 Bit 0 Compare Enable"]
pub type CF0CE5_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF0CR_SPEC, CF0CE5_A, O>;
impl<'a, const O: u8> CF0CE5_W<'a, O> {
    #[doc = "Comparison with bit 5 of Control Field 0 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF0CE5_A::_0)
    }
    #[doc = "Comparison with bit 5 of Control Field 0 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF0CE5_A::_1)
    }
}
#[doc = "Field `CF0CE6` reader - Control Field 6 Bit 0 Compare Enable"]
pub type CF0CE6_R = crate::BitReader<CF0CE6_A>;
#[doc = "Control Field 6 Bit 0 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF0CE6_A {
    #[doc = "0: Comparison with bit 6 of Control Field 0 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 6 of Control Field 0 is enabled."]
    _1 = 1,
}
impl From<CF0CE6_A> for bool {
    #[inline(always)]
    fn from(variant: CF0CE6_A) -> Self {
        variant as u8 != 0
    }
}
impl CF0CE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF0CE6_A {
        match self.bits {
            false => CF0CE6_A::_0,
            true => CF0CE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF0CE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF0CE6_A::_1
    }
}
#[doc = "Field `CF0CE6` writer - Control Field 6 Bit 0 Compare Enable"]
pub type CF0CE6_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF0CR_SPEC, CF0CE6_A, O>;
impl<'a, const O: u8> CF0CE6_W<'a, O> {
    #[doc = "Comparison with bit 6 of Control Field 0 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF0CE6_A::_0)
    }
    #[doc = "Comparison with bit 6 of Control Field 0 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF0CE6_A::_1)
    }
}
#[doc = "Field `CF0CE7` reader - Control Field 7 Bit 0 Compare Enable"]
pub type CF0CE7_R = crate::BitReader<CF0CE7_A>;
#[doc = "Control Field 7 Bit 0 Compare Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF0CE7_A {
    #[doc = "0: Comparison with bit 7 of Control Field 0 is disabled."]
    _0 = 0,
    #[doc = "1: Comparison with bit 7 of Control Field 0 is enabled."]
    _1 = 1,
}
impl From<CF0CE7_A> for bool {
    #[inline(always)]
    fn from(variant: CF0CE7_A) -> Self {
        variant as u8 != 0
    }
}
impl CF0CE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF0CE7_A {
        match self.bits {
            false => CF0CE7_A::_0,
            true => CF0CE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF0CE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF0CE7_A::_1
    }
}
#[doc = "Field `CF0CE7` writer - Control Field 7 Bit 0 Compare Enable"]
pub type CF0CE7_W<'a, const O: u8> = crate::BitWriter<'a, u8, CF0CR_SPEC, CF0CE7_A, O>;
impl<'a, const O: u8> CF0CE7_W<'a, O> {
    #[doc = "Comparison with bit 7 of Control Field 0 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF0CE7_A::_0)
    }
    #[doc = "Comparison with bit 7 of Control Field 0 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF0CE7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Control Field 0 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce0(&self) -> CF0CE0_R {
        CF0CE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control Field 1 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce1(&self) -> CF0CE1_R {
        CF0CE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control Field 2 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce2(&self) -> CF0CE2_R {
        CF0CE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control Field 3 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce3(&self) -> CF0CE3_R {
        CF0CE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control Field 4 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce4(&self) -> CF0CE4_R {
        CF0CE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Control Field 5 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce5(&self) -> CF0CE5_R {
        CF0CE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Control Field 6 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce6(&self) -> CF0CE6_R {
        CF0CE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Control Field 7 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce7(&self) -> CF0CE7_R {
        CF0CE7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control Field 0 Bit 0 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf0ce0(&mut self) -> CF0CE0_W<0> {
        CF0CE0_W::new(self)
    }
    #[doc = "Bit 1 - Control Field 1 Bit 0 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf0ce1(&mut self) -> CF0CE1_W<1> {
        CF0CE1_W::new(self)
    }
    #[doc = "Bit 2 - Control Field 2 Bit 0 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf0ce2(&mut self) -> CF0CE2_W<2> {
        CF0CE2_W::new(self)
    }
    #[doc = "Bit 3 - Control Field 3 Bit 0 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf0ce3(&mut self) -> CF0CE3_W<3> {
        CF0CE3_W::new(self)
    }
    #[doc = "Bit 4 - Control Field 4 Bit 0 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf0ce4(&mut self) -> CF0CE4_W<4> {
        CF0CE4_W::new(self)
    }
    #[doc = "Bit 5 - Control Field 5 Bit 0 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf0ce5(&mut self) -> CF0CE5_W<5> {
        CF0CE5_W::new(self)
    }
    #[doc = "Bit 6 - Control Field 6 Bit 0 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf0ce6(&mut self) -> CF0CE6_W<6> {
        CF0CE6_W::new(self)
    }
    #[doc = "Bit 7 - Control Field 7 Bit 0 Compare Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf0ce7(&mut self) -> CF0CE7_W<7> {
        CF0CE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Field 0 Compare Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cf0cr](index.html) module"]
pub struct CF0CR_SPEC;
impl crate::RegisterSpec for CF0CR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cf0cr::R](R) reader structure"]
impl crate::Readable for CF0CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cf0cr::W](W) writer structure"]
impl crate::Writable for CF0CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CF0CR to value 0"]
impl crate::Resettable for CF0CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
