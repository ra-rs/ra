#[doc = "Register `KRM` reader"]
pub struct R(crate::R<KRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KRM` writer"]
pub struct W(crate::W<KRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KRM_SPEC>;
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
impl From<crate::W<KRM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KRM0` reader - Key interrupt mode control 0"]
pub type KRM0_R = crate::BitReader<KRM0_A>;
#[doc = "Key interrupt mode control 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRM0_A {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<KRM0_A> for bool {
    #[inline(always)]
    fn from(variant: KRM0_A) -> Self {
        variant as u8 != 0
    }
}
impl KRM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRM0_A {
        match self.bits {
            false => KRM0_A::_0,
            true => KRM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRM0_A::_1
    }
}
#[doc = "Field `KRM0` writer - Key interrupt mode control 0"]
pub type KRM0_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KRM0_A, O>;
impl<'a, const O: u8> KRM0_W<'a, O> {
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRM0_A::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRM0_A::_1)
    }
}
#[doc = "Field `KRM1` reader - Key interrupt mode control 1"]
pub type KRM1_R = crate::BitReader<KRM1_A>;
#[doc = "Key interrupt mode control 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRM1_A {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<KRM1_A> for bool {
    #[inline(always)]
    fn from(variant: KRM1_A) -> Self {
        variant as u8 != 0
    }
}
impl KRM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRM1_A {
        match self.bits {
            false => KRM1_A::_0,
            true => KRM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRM1_A::_1
    }
}
#[doc = "Field `KRM1` writer - Key interrupt mode control 1"]
pub type KRM1_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KRM1_A, O>;
impl<'a, const O: u8> KRM1_W<'a, O> {
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRM1_A::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRM1_A::_1)
    }
}
#[doc = "Field `KRM2` reader - Key interrupt mode control 2"]
pub type KRM2_R = crate::BitReader<KRM2_A>;
#[doc = "Key interrupt mode control 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRM2_A {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<KRM2_A> for bool {
    #[inline(always)]
    fn from(variant: KRM2_A) -> Self {
        variant as u8 != 0
    }
}
impl KRM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRM2_A {
        match self.bits {
            false => KRM2_A::_0,
            true => KRM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRM2_A::_1
    }
}
#[doc = "Field `KRM2` writer - Key interrupt mode control 2"]
pub type KRM2_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KRM2_A, O>;
impl<'a, const O: u8> KRM2_W<'a, O> {
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRM2_A::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRM2_A::_1)
    }
}
#[doc = "Field `KRM3` reader - Key interrupt mode control 3"]
pub type KRM3_R = crate::BitReader<KRM3_A>;
#[doc = "Key interrupt mode control 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRM3_A {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<KRM3_A> for bool {
    #[inline(always)]
    fn from(variant: KRM3_A) -> Self {
        variant as u8 != 0
    }
}
impl KRM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRM3_A {
        match self.bits {
            false => KRM3_A::_0,
            true => KRM3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRM3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRM3_A::_1
    }
}
#[doc = "Field `KRM3` writer - Key interrupt mode control 3"]
pub type KRM3_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KRM3_A, O>;
impl<'a, const O: u8> KRM3_W<'a, O> {
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRM3_A::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRM3_A::_1)
    }
}
#[doc = "Field `KRM4` reader - Key interrupt mode control 4"]
pub type KRM4_R = crate::BitReader<KRM4_A>;
#[doc = "Key interrupt mode control 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRM4_A {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<KRM4_A> for bool {
    #[inline(always)]
    fn from(variant: KRM4_A) -> Self {
        variant as u8 != 0
    }
}
impl KRM4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRM4_A {
        match self.bits {
            false => KRM4_A::_0,
            true => KRM4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRM4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRM4_A::_1
    }
}
#[doc = "Field `KRM4` writer - Key interrupt mode control 4"]
pub type KRM4_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KRM4_A, O>;
impl<'a, const O: u8> KRM4_W<'a, O> {
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRM4_A::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRM4_A::_1)
    }
}
#[doc = "Field `KRM5` reader - Key interrupt mode control 5"]
pub type KRM5_R = crate::BitReader<KRM5_A>;
#[doc = "Key interrupt mode control 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRM5_A {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<KRM5_A> for bool {
    #[inline(always)]
    fn from(variant: KRM5_A) -> Self {
        variant as u8 != 0
    }
}
impl KRM5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRM5_A {
        match self.bits {
            false => KRM5_A::_0,
            true => KRM5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRM5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRM5_A::_1
    }
}
#[doc = "Field `KRM5` writer - Key interrupt mode control 5"]
pub type KRM5_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KRM5_A, O>;
impl<'a, const O: u8> KRM5_W<'a, O> {
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRM5_A::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRM5_A::_1)
    }
}
#[doc = "Field `KRM6` reader - Key interrupt mode control 6"]
pub type KRM6_R = crate::BitReader<KRM6_A>;
#[doc = "Key interrupt mode control 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRM6_A {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<KRM6_A> for bool {
    #[inline(always)]
    fn from(variant: KRM6_A) -> Self {
        variant as u8 != 0
    }
}
impl KRM6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRM6_A {
        match self.bits {
            false => KRM6_A::_0,
            true => KRM6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRM6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRM6_A::_1
    }
}
#[doc = "Field `KRM6` writer - Key interrupt mode control 6"]
pub type KRM6_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KRM6_A, O>;
impl<'a, const O: u8> KRM6_W<'a, O> {
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRM6_A::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRM6_A::_1)
    }
}
#[doc = "Field `KRM7` reader - Key interrupt mode control 7"]
pub type KRM7_R = crate::BitReader<KRM7_A>;
#[doc = "Key interrupt mode control 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRM7_A {
    #[doc = "0: Does not detect key interrupt signal"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signal."]
    _1 = 1,
}
impl From<KRM7_A> for bool {
    #[inline(always)]
    fn from(variant: KRM7_A) -> Self {
        variant as u8 != 0
    }
}
impl KRM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRM7_A {
        match self.bits {
            false => KRM7_A::_0,
            true => KRM7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRM7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRM7_A::_1
    }
}
#[doc = "Field `KRM7` writer - Key interrupt mode control 7"]
pub type KRM7_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KRM7_A, O>;
impl<'a, const O: u8> KRM7_W<'a, O> {
    #[doc = "Does not detect key interrupt signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRM7_A::_0)
    }
    #[doc = "Detect key interrupt signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRM7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Key interrupt mode control 0"]
    #[inline(always)]
    pub fn krm0(&self) -> KRM0_R {
        KRM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key interrupt mode control 1"]
    #[inline(always)]
    pub fn krm1(&self) -> KRM1_R {
        KRM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key interrupt mode control 2"]
    #[inline(always)]
    pub fn krm2(&self) -> KRM2_R {
        KRM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Key interrupt mode control 3"]
    #[inline(always)]
    pub fn krm3(&self) -> KRM3_R {
        KRM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Key interrupt mode control 4"]
    #[inline(always)]
    pub fn krm4(&self) -> KRM4_R {
        KRM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Key interrupt mode control 5"]
    #[inline(always)]
    pub fn krm5(&self) -> KRM5_R {
        KRM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Key interrupt mode control 6"]
    #[inline(always)]
    pub fn krm6(&self) -> KRM6_R {
        KRM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Key interrupt mode control 7"]
    #[inline(always)]
    pub fn krm7(&self) -> KRM7_R {
        KRM7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key interrupt mode control 0"]
    #[inline(always)]
    #[must_use]
    pub fn krm0(&mut self) -> KRM0_W<0> {
        KRM0_W::new(self)
    }
    #[doc = "Bit 1 - Key interrupt mode control 1"]
    #[inline(always)]
    #[must_use]
    pub fn krm1(&mut self) -> KRM1_W<1> {
        KRM1_W::new(self)
    }
    #[doc = "Bit 2 - Key interrupt mode control 2"]
    #[inline(always)]
    #[must_use]
    pub fn krm2(&mut self) -> KRM2_W<2> {
        KRM2_W::new(self)
    }
    #[doc = "Bit 3 - Key interrupt mode control 3"]
    #[inline(always)]
    #[must_use]
    pub fn krm3(&mut self) -> KRM3_W<3> {
        KRM3_W::new(self)
    }
    #[doc = "Bit 4 - Key interrupt mode control 4"]
    #[inline(always)]
    #[must_use]
    pub fn krm4(&mut self) -> KRM4_W<4> {
        KRM4_W::new(self)
    }
    #[doc = "Bit 5 - Key interrupt mode control 5"]
    #[inline(always)]
    #[must_use]
    pub fn krm5(&mut self) -> KRM5_W<5> {
        KRM5_W::new(self)
    }
    #[doc = "Bit 6 - Key interrupt mode control 6"]
    #[inline(always)]
    #[must_use]
    pub fn krm6(&mut self) -> KRM6_W<6> {
        KRM6_W::new(self)
    }
    #[doc = "Bit 7 - Key interrupt mode control 7"]
    #[inline(always)]
    #[must_use]
    pub fn krm7(&mut self) -> KRM7_W<7> {
        KRM7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KEY Return Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [krm](index.html) module"]
pub struct KRM_SPEC;
impl crate::RegisterSpec for KRM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [krm::R](R) reader structure"]
impl crate::Readable for KRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [krm::W](W) writer structure"]
impl crate::Writable for KRM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KRM to value 0"]
impl crate::Resettable for KRM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
