#[doc = "Register `KRF` reader"]
pub struct R(crate::R<KRF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KRF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KRF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KRF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KRF` writer"]
pub struct W(crate::W<KRF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KRF_SPEC>;
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
impl From<crate::W<KRF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KRF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KRF0` reader - Key interrupt flag 0\n\nThe field is **modified** in some way after a read operation."]
pub type KRF0_R = crate::BitReader<KRF0_A>;
#[doc = "Key interrupt flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF0_A {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<KRF0_A> for bool {
    #[inline(always)]
    fn from(variant: KRF0_A) -> Self {
        variant as u8 != 0
    }
}
impl KRF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRF0_A {
        match self.bits {
            false => KRF0_A::_0,
            true => KRF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF0_A::_1
    }
}
#[doc = "Field `KRF0` writer - Key interrupt flag 0"]
pub type KRF0_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, KRF_SPEC, KRF0_A, O>;
impl<'a, const O: u8> KRF0_W<'a, O> {
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRF0_A::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRF0_A::_1)
    }
}
#[doc = "Field `KRF1` reader - Key interrupt flag 1\n\nThe field is **modified** in some way after a read operation."]
pub type KRF1_R = crate::BitReader<KRF1_A>;
#[doc = "Key interrupt flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF1_A {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<KRF1_A> for bool {
    #[inline(always)]
    fn from(variant: KRF1_A) -> Self {
        variant as u8 != 0
    }
}
impl KRF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRF1_A {
        match self.bits {
            false => KRF1_A::_0,
            true => KRF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF1_A::_1
    }
}
#[doc = "Field `KRF1` writer - Key interrupt flag 1"]
pub type KRF1_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, KRF_SPEC, KRF1_A, O>;
impl<'a, const O: u8> KRF1_W<'a, O> {
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRF1_A::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRF1_A::_1)
    }
}
#[doc = "Field `KRF2` reader - Key interrupt flag 2\n\nThe field is **modified** in some way after a read operation."]
pub type KRF2_R = crate::BitReader<KRF2_A>;
#[doc = "Key interrupt flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF2_A {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<KRF2_A> for bool {
    #[inline(always)]
    fn from(variant: KRF2_A) -> Self {
        variant as u8 != 0
    }
}
impl KRF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRF2_A {
        match self.bits {
            false => KRF2_A::_0,
            true => KRF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF2_A::_1
    }
}
#[doc = "Field `KRF2` writer - Key interrupt flag 2"]
pub type KRF2_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, KRF_SPEC, KRF2_A, O>;
impl<'a, const O: u8> KRF2_W<'a, O> {
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRF2_A::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRF2_A::_1)
    }
}
#[doc = "Field `KRF3` reader - Key interrupt flag 3\n\nThe field is **modified** in some way after a read operation."]
pub type KRF3_R = crate::BitReader<KRF3_A>;
#[doc = "Key interrupt flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF3_A {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<KRF3_A> for bool {
    #[inline(always)]
    fn from(variant: KRF3_A) -> Self {
        variant as u8 != 0
    }
}
impl KRF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRF3_A {
        match self.bits {
            false => KRF3_A::_0,
            true => KRF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF3_A::_1
    }
}
#[doc = "Field `KRF3` writer - Key interrupt flag 3"]
pub type KRF3_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, KRF_SPEC, KRF3_A, O>;
impl<'a, const O: u8> KRF3_W<'a, O> {
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRF3_A::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRF3_A::_1)
    }
}
#[doc = "Field `KRF4` reader - Key interrupt flag 4\n\nThe field is **modified** in some way after a read operation."]
pub type KRF4_R = crate::BitReader<KRF4_A>;
#[doc = "Key interrupt flag 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF4_A {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<KRF4_A> for bool {
    #[inline(always)]
    fn from(variant: KRF4_A) -> Self {
        variant as u8 != 0
    }
}
impl KRF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRF4_A {
        match self.bits {
            false => KRF4_A::_0,
            true => KRF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF4_A::_1
    }
}
#[doc = "Field `KRF4` writer - Key interrupt flag 4"]
pub type KRF4_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, KRF_SPEC, KRF4_A, O>;
impl<'a, const O: u8> KRF4_W<'a, O> {
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRF4_A::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRF4_A::_1)
    }
}
#[doc = "Field `KRF5` reader - Key interrupt flag 5\n\nThe field is **modified** in some way after a read operation."]
pub type KRF5_R = crate::BitReader<KRF5_A>;
#[doc = "Key interrupt flag 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF5_A {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<KRF5_A> for bool {
    #[inline(always)]
    fn from(variant: KRF5_A) -> Self {
        variant as u8 != 0
    }
}
impl KRF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRF5_A {
        match self.bits {
            false => KRF5_A::_0,
            true => KRF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF5_A::_1
    }
}
#[doc = "Field `KRF5` writer - Key interrupt flag 5"]
pub type KRF5_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, KRF_SPEC, KRF5_A, O>;
impl<'a, const O: u8> KRF5_W<'a, O> {
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRF5_A::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRF5_A::_1)
    }
}
#[doc = "Field `KRF6` reader - Key interrupt flag 6\n\nThe field is **modified** in some way after a read operation."]
pub type KRF6_R = crate::BitReader<KRF6_A>;
#[doc = "Key interrupt flag 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF6_A {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<KRF6_A> for bool {
    #[inline(always)]
    fn from(variant: KRF6_A) -> Self {
        variant as u8 != 0
    }
}
impl KRF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRF6_A {
        match self.bits {
            false => KRF6_A::_0,
            true => KRF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF6_A::_1
    }
}
#[doc = "Field `KRF6` writer - Key interrupt flag 6"]
pub type KRF6_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, KRF_SPEC, KRF6_A, O>;
impl<'a, const O: u8> KRF6_W<'a, O> {
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRF6_A::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRF6_A::_1)
    }
}
#[doc = "Field `KRF7` reader - Key interrupt flag 7\n\nThe field is **modified** in some way after a read operation."]
pub type KRF7_R = crate::BitReader<KRF7_A>;
#[doc = "Key interrupt flag 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF7_A {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected."]
    _1 = 1,
}
impl From<KRF7_A> for bool {
    #[inline(always)]
    fn from(variant: KRF7_A) -> Self {
        variant as u8 != 0
    }
}
impl KRF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRF7_A {
        match self.bits {
            false => KRF7_A::_0,
            true => KRF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF7_A::_1
    }
}
#[doc = "Field `KRF7` writer - Key interrupt flag 7"]
pub type KRF7_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, KRF_SPEC, KRF7_A, O>;
impl<'a, const O: u8> KRF7_W<'a, O> {
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRF7_A::_0)
    }
    #[doc = "Interrupt detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRF7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Key interrupt flag 0"]
    #[inline(always)]
    pub fn krf0(&self) -> KRF0_R {
        KRF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key interrupt flag 1"]
    #[inline(always)]
    pub fn krf1(&self) -> KRF1_R {
        KRF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key interrupt flag 2"]
    #[inline(always)]
    pub fn krf2(&self) -> KRF2_R {
        KRF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Key interrupt flag 3"]
    #[inline(always)]
    pub fn krf3(&self) -> KRF3_R {
        KRF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Key interrupt flag 4"]
    #[inline(always)]
    pub fn krf4(&self) -> KRF4_R {
        KRF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Key interrupt flag 5"]
    #[inline(always)]
    pub fn krf5(&self) -> KRF5_R {
        KRF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Key interrupt flag 6"]
    #[inline(always)]
    pub fn krf6(&self) -> KRF6_R {
        KRF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Key interrupt flag 7"]
    #[inline(always)]
    pub fn krf7(&self) -> KRF7_R {
        KRF7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key interrupt flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn krf0(&mut self) -> KRF0_W<0> {
        KRF0_W::new(self)
    }
    #[doc = "Bit 1 - Key interrupt flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn krf1(&mut self) -> KRF1_W<1> {
        KRF1_W::new(self)
    }
    #[doc = "Bit 2 - Key interrupt flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn krf2(&mut self) -> KRF2_W<2> {
        KRF2_W::new(self)
    }
    #[doc = "Bit 3 - Key interrupt flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn krf3(&mut self) -> KRF3_W<3> {
        KRF3_W::new(self)
    }
    #[doc = "Bit 4 - Key interrupt flag 4"]
    #[inline(always)]
    #[must_use]
    pub fn krf4(&mut self) -> KRF4_W<4> {
        KRF4_W::new(self)
    }
    #[doc = "Bit 5 - Key interrupt flag 5"]
    #[inline(always)]
    #[must_use]
    pub fn krf5(&mut self) -> KRF5_W<5> {
        KRF5_W::new(self)
    }
    #[doc = "Bit 6 - Key interrupt flag 6"]
    #[inline(always)]
    #[must_use]
    pub fn krf6(&mut self) -> KRF6_W<6> {
        KRF6_W::new(self)
    }
    #[doc = "Bit 7 - Key interrupt flag 7"]
    #[inline(always)]
    #[must_use]
    pub fn krf7(&mut self) -> KRF7_W<7> {
        KRF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KEY Return Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [krf](index.html) module\n\nThe register is **modified** in some way after a read operation."]
pub struct KRF_SPEC;
impl crate::RegisterSpec for KRF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [krf::R](R) reader structure"]
impl crate::Readable for KRF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [krf::W](W) writer structure"]
impl crate::Writable for KRF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xff;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KRF to value 0"]
impl crate::Resettable for KRF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
